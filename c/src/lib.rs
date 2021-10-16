//! The `browser-window-c` crate provides a C interface to any browser engine - and windowing API implementations that are not available in Rust.
//! This is not called `browser-window-sys`, because it is not a FFI binding crate of one external C library.
//! 
//! Each 'type object' has a handle type, which is supposed to be used as a pointer. (E.g. `bw_Application*` for the application functionality.)
//! Each handle type makes use of an 'subtype' postfixed with `Impl`, like `bw_ApplicationImpl`, which is a member of `bw_Application` itself.
//! Within this implementations' subtype can be stored any variables that that implementation needs to hold.
//! 
//! These implementations are then written in code files contained within a directory of the main type object.
//! So for example, the `application` directory contains `win32.c`, `gtk.c`, `cef.cpp` and `cef_window.cpp`, which all implement a part of the application type if their corresponding implementation is selected.
//! Then, there is also a `common.c` file in that directory, which implements any functionality that is implemented the same for any implementation.
//! So there is no need for redundancy.

mod bindings;

use std::{
	error::Error,
	ffi::CStr,
	fmt,
	ptr,
	slice,
	str
};




pub use crate::bindings::*;



/**************************************************************
 * Implementations for C structs that are also useful in Rust *
 **************************************************************/

impl cbw_CStrSlice {
	pub fn empty() -> Self {
		Self { len: 0, data: ptr::null() }
	}
}

impl From<&str> for cbw_CStrSlice {
	fn from( string: &str ) -> Self {
		Self {
			data: string.as_bytes().as_ptr() as _,
			len: string.len() as _
		}
	}
}

impl<'a> Into<&'a str> for cbw_CStrSlice {
	fn into( self ) -> &'a str {
		let raw: &[u8] = unsafe { slice::from_raw_parts(self.data as _, self.len as _ ) };

		#[cfg(debug_assertions)]
			return str::from_utf8( raw ).expect("Invalid UTF-8");
		#[cfg(not(debug_assertions))]
			return unsafe { str::from_utf8_unchecked( raw ) };
	}
}

impl Into<String> for cbw_CStrSlice {
	fn into( self ) -> String {
		let str: &str = self.into();

		str.to_owned()
	}
}



impl cbw_StrSlice {
	pub fn empty() -> Self {
		Self { len: 0, data: ptr::null_mut() }
	}
}

impl<'a> Into<&'a str> for cbw_StrSlice {
	fn into( self ) -> &'a str {
		let raw: &[u8] = unsafe { slice::from_raw_parts(self.data as _, self.len as _ ) };

		#[cfg(debug_assertions)]
			return str::from_utf8( raw ).expect("Invalid UTF-8");
		#[cfg(not(debug_assertions))]
			return unsafe { str::from_utf8_unchecked( raw ) };
	}
}

impl Into<String> for cbw_StrSlice {
	fn into( self ) -> String {
		let str: &str = self.into();

		str.to_owned()
	}
}

impl Error for cbw_Err {
	fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}

impl fmt::Display for cbw_Err {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

		unsafe {
			let msg_ptr = (self.alloc_message.unwrap())( self.code, self.data );
			let cstr = CStr::from_ptr( msg_ptr );
			write!(f, "{}", cstr.to_string_lossy().as_ref())
		}
	}
}