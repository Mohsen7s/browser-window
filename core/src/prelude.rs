use browser_window_c::*;

pub use crate::cookie::*;
// TODO: Wrap this type as well
pub use browser_window_c::cbw_WindowOptions;



pub struct Dims2D(pub(crate) cbw_Dims2D);
pub struct Pos2D(pub(crate) cbw_Pos2D);



pub use crate::application::{ApplicationCore, ApplicationImpl};
pub use crate::browser_window::{BrowserWindowCore, BrowserWindowImpl};
pub use crate::window::{WindowCore, WindowImpl};



impl Dims2D {
	pub fn new(width: u16, height: u16) -> Dims2D {
		Dims2D(cbw_Dims2D { width: width, height: height })
	}

	pub fn width(&self) -> u16	{ self.0.width }
	pub fn height(&self) -> u16	{ self.0.height }
}

impl Pos2D {
	pub fn new(x: u16, y: u16) -> Pos2D {
		Pos2D(cbw_Pos2D {x: x, y: y})
	}

	pub fn x(&self) -> u16	{ self.0.x }
	pub fn y(&self) -> u16	{ self.0.y }
}