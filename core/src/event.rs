//! This module contains all event related types.



use std::{
	boxed::Box,
	collections::LinkedList,
	future::Future,
	pin::Pin
};


#[doc(hidden)]
#[macro_export]
macro_rules! def_event_c_handler {
	( $event_name:ident ( $handle_name:ident : $c_handle_type:ty as $handle_type:ty, $($arg:ident: $arg_type:ty),* ) $content:block ) => {
		use concat_idents::concat_idents;

		concat_idents!(fn_name = ffi_event_, $event_name, _handler {
			unsafe extern "C" fn fn_name(inner_handle: *mut $c_handle_type, $($arg: $arg_type),* ) {
				let data_ptr = (*inner_handle).user_data as *mut UserData;
				let data = &mut *data_ptr;
			
				if data.events.$event_name.is_registered() {
					let $handle_name = $handle_type { inner: inner_handle };

					let output = $content;

					data.events.$event_name.invoke($handle_name, &output);
				}
			}
		});
	};
}



/// A (boxed) closure that will be called when the corresponding event is being fired.
/// 
/// *Note:* When and only when feature `threadsafe` is set, the closure needs to be `Send`.
#[cfg(feature = "threadsafe")]
type EventHandler<'a,T,A> = Box<dyn FnMut( T, &A ) -> Pin<Box<dyn Future<Output=()> + 'a>> + Send + 'a>;
#[cfg(not(feature = "threadsafe"))]
type EventHandler<'a,T,A> = Box<dyn FnMut( T, &A ) -> Pin<Box<dyn Future<Output=()> + 'a>> + 'a>;

/// Is able to store callbacks, and invokes them all when the event is fired.
pub struct Event<'a,T,A> {
	handlers: LinkedList<EventHandler<'a,T,A>>
}



impl<'a,T,A> Event<'a,T,A> where T: Clone {

	pub(in crate) fn is_registered(&self) -> bool {
		self.handlers.len() > 0
	}

	/// Invokes the event, which calls all handlers that have been registered to this event.
	pub(in crate) fn invoke( &mut self, handle: T, args: &A  ) {

		for h in self.handlers.iter_mut() {
			h( handle.clone(), args );
		}
	}

	/// Register a closure to be invoked for this event.
	#[cfg(not(feature = "threadsafe"))]
	pub fn register<H>( &mut self, mut handler: H ) where
		H: FnMut( T, &A ) + 'a
	{
		self.handlers.push_back(Box::new(move |handle, args| {
			handler( handle, args );
			Box::pin(async {})
		}));
	}

	/// Register a closure to be invoked for this event.
	#[cfg(feature = "threadsafe")]
	pub fn register<H>( &mut self, mut handler: H ) where
		H: FnMut( T, &A ) + Send + 'a
	{
		self.handlers.push_back(Box::new(move |handle, args| {
			handler( handle, args );
			Box::pin(async {})
		}));
	}

	/// Register an 'async closure' to be invoked for this event.
	///
	/// # Example
	/// ```ignore
	/// my_event.register_async(|args| async move {
	///     // Do something ...
	/// });
	/// ```
	#[cfg(not(feature = "threadsafe"))]
	pub fn register_async<H,F>( &mut self, mut handler: H ) where
		H: FnMut( T, &A ) -> F + 'a,
		F: Future<Output=()> + 'a
	{
		self.handlers.push_back(Box::new(move |handle, args| Box::pin( handler( handle, args ) ) ) );
	}

	/// Register an 'async closure' to be invoked for this event.
	///
	/// # Example
	/// ```ignore
	/// my_event.register_async(|args| async move {
	///     // Do something ...
	/// });
	/// ```
	#[cfg(feature = "threadsafe")]
	pub fn register_async<H,F>( &mut self, mut handler: H ) where
		H: FnMut( T, &A ) -> F + Send + 'a,
		F: Future<Output=()> + 'a
	{
		self.handlers.push_back(Box::new(move |handle, args| Box::pin( handler( handle, args ) ) ) );
	}
}

impl<'a,T,A> Default for Event<'a,T,A> {
	
	fn default() -> Self {
		Self {
			handlers: LinkedList::new()
		}
	}
}