use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(PartialOrd, PartialEq)]
#[derive(Debug)]
pub struct BhomzError {
	msg: String,
}

impl From<String> for BhomzError {
	fn from(value: String) -> Self {
		Self { msg: value }
	}
}

impl Display for BhomzError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		f.write_str(&self.msg)
	}
}

impl Error for BhomzError {}

#[inline(always)]
pub fn bhomz_error(msg: impl ToString) -> BhomzError {
	BhomzError {
		msg: msg.to_string(),
	}
}

#[inline(always)]
pub fn bhomz_error_borrow(msg: &impl ToString) -> BhomzError {
	BhomzError {
		msg: msg.to_string(),
	}
}

pub type BhomzResult<T> = Result<T, BhomzError>;
pub type BhomzThrowable = BhomzResult<()>;
