use std::fmt::{Display, Formatter, Pointer};

pub trait ToCCharPtr
where
	Self: ToString,
{
	fn to_c_char_ptr(&self) -> *const std::os::raw::c_char {
		std::ffi::CString::new(self.to_string().as_str())
			.unwrap()
			.into_raw()
	}
}

impl<T: ToString> ToCCharPtr for T {}

pub struct Printable {
	s: *const std::os::raw::c_char,
}

impl Printable {
	pub fn new(s: *const std::os::raw::c_char) -> Self {
		Self { s }
	}
}

impl Display for Printable {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if self.s.is_null() {
			write!(f, "null")
		} else {
			self.s.fmt(f)
		}
	}
}
