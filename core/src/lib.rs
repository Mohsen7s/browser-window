//! The browser-window-core crate implements all browser - and window related functionality, for any supported platform, and any supported browser engine.
//! It is essentially a 'wrapping' library for different implementations.
//! 
//! The way this is done, is by providing a trait for each 'type object' to implement, and then implementing it for each implementation.
//! Each trait is postfixed with `Core`, making them distinguisable from their corresponding structs in browser-window.
//! Each trait implementation is postfixed with `Impl`.
//! So for example, the `BrowserWindow` struct in the `browser-window` crate makes use of the `BrowserWindowCore` trait with the `BrowserWindowImpl` implementation.
//!
//! There are basically two API that are being wrapped. The browser engine or - embedding API, and the windowing API.
//! The `browser_window` and `cookie` traits are being implemented by the browser embedding API,
//!  the `window` traits are being implemented by the windowing API,
//!  and the `application` trait may be implemented by both.
//! The following features specifiy which API to use.
//! - `cef`: Use CEF3 as the browser embedding API. (the default)
//! - `cef-window`: Use CEF3's own internal windowing API. (default on non-Windows platforms)
//! - `gtk`: Use GTK as the windowing API. (currently broken)
//! If `cef-window` is not chosen, it will try to use the platform's internal windowing API.
//! For Windows this would be the win32 API, and for macOS this would be cocoa.
//! Except that cocoa is not implemented yet and thus still defaults to the `cef-window` option.
//!
//! Now because most implementations are made in C, there are `browser-window-core` implementations for each 'object' that simply wrap C code contained in `browser-window-c`.
//! `browser-window-c` has its own structure to order different implementations.
//! How that is done is not important within the `browser-window-core` crate, the C implementations merely invoke the corresponding C FFI functions.
//! 
//! An interresting note is that because both `browser-window-c` and `browser-window-core` have their own system of wrapping underlying implementations,
//!  it introduces a little overhead.
//! Each 'wrapping' layer can be seen as a layer that introduces it.
//! However, this has been done so that implementations can be written in Rust as well as in C.
//! Most browser engines - and (UI) windowing API's will be available in C, or something that can be wrapped by C (like C++ & Objective-C).
//! But because Servo embedding support is planned for somewhere in the future, and Servo is a Rust API, we need Rust support as well.
//! So that being said, it is still nice to know that any Rust implementation enjoys the lowest overhead from this wrapping layer, if there would be any at all.

pub mod application;
pub mod browser_window;
pub mod cookie;
pub mod error;
pub mod prelude;
pub mod window;