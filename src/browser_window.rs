use boxfnonce::SendBoxFnOnce;
use browser_window_ffi::*;
use futures_channel::oneshot;
use std::{
	error::Error,
	ffi::CStr,
	ops::Deref,
	os::raw::*,
	rc::Rc,
	sync::Arc
};

use crate::application::*;
use crate::common::*;

pub mod builder;

pub use builder::BrowserWindowBuilder;



/// A thread-unsafe handle to a browser window.
// A reference counter is held internally,
//     meaning that you can simply clone this handle without having to worry about memory leakage.
// If the user closes the window, this handle stays valid.
// Also, if you lose this handle, window destruction and cleanup is only done when the user actually closes it.
// So you don't have to worry about lifetimes and/or propper destruction of the window either.
#[derive(Clone)]
pub struct BrowserWindow {
	pub(in super) inner: Rc<BrowserWindowInner>
}

/// A thread-safe handle to a browser window.
/// It allows you to dispatch code to the GUI thread.
// It provides the same functionality as Browserwindow.
// However, each function is async: it runs on the GUI thread, and returns when it is done.
#[derive(Clone)]
pub struct BrowserWindowAsync {
	pub(in super) inner: Arc<BrowserWindowInnerAsync>
}

/// A handle to a browser window.
/// This can not be instantiated, but can be seen as an interface that is provided for by the BrowserWindow and BrowserWindowAsync 'handles'.
#[derive(Clone)]
pub struct BrowserWindowHandle {
	_ffi_handle: *mut bw_BrowserWindow
}
// BrowserWindowHandle is made to be Send and Sync, because it is used internally by BrowserWindowAsync as well.
unsafe impl Send for BrowserWindowHandle {}
unsafe impl Sync for BrowserWindowHandle {}

/// This structure holds a browser window handle.
/// The purpose of this structure is to invoke the FFI function to drop the browser window handle, when this struct is dropped naturally by Rust.
/// So by putting this struct in an Arc<...> or Rc<...>, you effectively have some sort garbage collection.
pub struct BrowserWindowInner {
	app: Arc<ApplicationInner>,	// Keep a reference to ApplicationInner so that the application handle doesn't get freed prematurely
	pub(in super) handle: BrowserWindowHandle
}

pub struct BrowserWindowInnerAsync {
	app: Arc<ApplicationInner>,	// Keep a reference to ApplicationInner so that the application handle doesn't get freed prematurely
	pub(in super) handle: BrowserWindowHandle
}



impl HasAppHandle for BrowserWindow {
	fn app_handle( &self ) -> ApplicationHandle {
		self.inner.app.handle.clone()
	}
}



impl Deref for BrowserWindow {
	type Target = BrowserWindowHandle;

	fn deref( &self ) -> &Self::Target {
		&self.inner.handle
	}
}



impl BrowserWindowAsync {

	/// Closes the browser.
	pub async fn close( self ) {
		self.dispatch(|bw| {
			bw.close()
		}).await;
	}

	/// Executes the given closure within the GUI thread
	///
	/// # Arguments
	/// * `func` - The closure to run on the GUI thread
	pub fn dispatch<'a,F,R>( &self, func: F ) -> BrowserWindowDispatchFuture<'a,R> where
		F: FnOnce( BrowserWindowHandle ) -> R + Send + 'a,
		R: Send
	{
		BrowserWindowDispatchFuture::new( self.inner.handle.clone(), func )
	}

	/// Executes the given javascript code, and returns the resulting output as a string when done.
	///
	/// # Arguments:
	/// * `js` - Javascript code
	pub async fn eval_js( &self, js: &str ) -> Result<String, Box<dyn Error + Send>> {

		let (tx, rx) = oneshot::channel::<Result<String, Box<dyn Error + Send>>>();

		self.dispatch(|bw| {

			// Executing the JavaScript on the GUI thread
			bw.eval_js( js, |_, result| {

				// Result is ready
				let _ = tx.send( result ).unwrap();
			} );
		}).await;

		// The result
		rx.await.unwrap()
	}

	/// Causes the browser to navigate to the given url.
	///
	/// # Arguments
	/// * `url` - The url to navigate to
	pub async fn navigate( &self, url: &str ) -> Result<(), Box<dyn Error + Send>> {
		*self.dispatch(|bw| {
			bw.navigate( url )
		}).await
	}
}

impl Deref for BrowserWindowAsync {
	type Target = BrowserWindowHandle;

	fn deref( &self ) -> &Self::Target {
		&self.inner.handle
	}
}



type BrowserWindowCallbackData<'a> = SendBoxFnOnce<'a,(BrowserWindowHandle, Result<String, Box<dyn Error + Send>>),()>;

/// The future that dispatches a closure on the GUI thread used by BrowserWindowAsync.
pub type BrowserWindowDispatchFuture<'a,R> = DispatchFuture<'a, BrowserWindowHandle, R>;



impl BrowserWindowHandle {

	/// Closes the browser.
	// The browser will be freed from memory when the last handle to it gets dropped.
	pub fn close( self ) {
		unsafe { bw_BrowserWindow_close( self._ffi_handle ); }
	}

	/// Executes the given javascript code, and returns the output via a callback.
	/// If you don't need the result, see "exec_js".
	///
	/// # Arguments:
	/// * `js` - The javascript code to execute.
	/// * `on_complete` - The 'callback'. This closure will be invoked, with the result provided as the first argument.
	///                   The result contains the output of the javascript code when it succeeded.
	///                   Otherwise the error explains the javascript exception.
	pub fn eval_js<'a,H>( &self, js: &str, on_complete: H ) where
		H: FnOnce( BrowserWindowHandle, Result<String, Box<dyn Error + Send>> ) + Send + 'a
	{
		let data_ptr = Box::into_raw( Box::new(
			BrowserWindowCallbackData::<'a>::new( on_complete )
		) );

		unsafe { bw_BrowserWindow_evalJs(
			self._ffi_handle,
			js.into(),
			ffi_eval_js_callback,
			data_ptr as _
		) };
	}

	/// Executes the given javascript code without waiting on it to finish.
	///
	/// # Arguments:
	/// * `js` - The javascript code
	pub fn exec_js( &self, js: &str ) {
		self.eval_js( js, |_,_|{} );
	}

	unsafe fn from_ptr( ptr: *mut bw_BrowserWindow ) -> Self {
		Self {
			_ffi_handle: ptr
		}
	}

	/// Causes the browser to navigate to the given url.
	///
	/// # Arguments
	/// * `url` - The url to navigate to
	pub fn navigate( &self, url: &str ) -> Result<(), Box<dyn Error + Send>> {
		let err = unsafe { bw_BrowserWindow_navigate( self._ffi_handle, url.into() ) };

		if err.code == 0 {
			return Ok(());
		}

		Err( Box::new( err ) )
	}
}

impl HasAppHandle for BrowserWindowHandle {
	fn app_handle( &self ) -> ApplicationHandle {
		ApplicationHandle {
			_ffi_handle: unsafe { bw_BrowserWindow_getApp( self._ffi_handle ) as _ }
		}
	}
}

impl Deref for BrowserWindowInner {
	type Target = BrowserWindowHandle;

	fn deref( &self ) -> &Self::Target {
		&self.handle
	}
}

impl Drop for BrowserWindowInner {
	fn drop( &mut self ) {
		unsafe { bw_BrowserWindow_drop( self.handle._ffi_handle ) }
	}
}

impl Drop for BrowserWindowInnerAsync {
	fn drop( &mut self ) {
		unsafe { bw_Application_dispatch( self.app.handle._ffi_handle, ffi_free_browser_window, self.handle._ffi_handle as _ ); }
	}
}



extern "C" fn ffi_free_browser_window( _app: *mut bw_Application, data: *mut c_void ) {
	unsafe { bw_BrowserWindow_drop( data as *mut bw_BrowserWindow ); }
}

extern "C" fn ffi_eval_js_callback( bw: *mut bw_BrowserWindow, cb_data: *mut c_void, result: *const c_char, error: *const bw_Err ) {

	let data_ptr = cb_data as *mut BrowserWindowCallbackData;
	let data = unsafe { Box::from_raw( data_ptr ) };

	// Construct a result value depending on whether the result or error parameters are set
	let result_val: Result<String, Box<dyn Error + Send>> = if error.is_null() {
		let result_str = unsafe { CStr::from_ptr( result ).to_string_lossy().to_owned().to_string() };
		Ok( result_str )
	}
	else {
		Err( Box::new( unsafe { (*error).clone() } ) )
	};

	// Construct a
	let handle = unsafe { BrowserWindowHandle::from_ptr( bw ) };

	data.call( handle, result_val );
}
