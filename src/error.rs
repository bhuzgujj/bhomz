pub use aliases::*;

/// Shortcut to void error result
pub type BhomzThrowable = BhomzResult<()>;

#[cfg(not(feature = "anyhow"))]
pub mod aliases {
	use std::error::Error;
	use std::fmt::{Display, Formatter};

	/// Default error type
	pub type BhomzError = BhomzErrorImpl;
	/// Default result type shortcut
	pub type BhomzResult<T> = Result<T, BhomzErrorImpl>;

	#[derive(PartialOrd, PartialEq)]
	#[derive(Debug)]
	pub struct BhomzErrorImpl {
		msg: String,
	}

	impl From<String> for BhomzErrorImpl {
		fn from(value: String) -> Self {
			Self { msg: value }
		}
	}

	impl Display for BhomzErrorImpl {
		fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
			f.write_str(&self.msg)
		}
	}

	impl Error for BhomzErrorImpl {}

	#[inline(always)]
	pub(crate) fn bhomz_error(msg: impl ToString) -> BhomzError {
		BhomzError {
			msg: msg.to_string(),
		}
	}
}

#[cfg(feature = "anyhow")]
pub mod aliases {
	/// Anyhow Error
	pub type BhomzError = anyhow::Error;
	/// Anyhow Result
	pub type BhomzResult<T> = anyhow::Result<T>;

	#[inline(always)]
	pub(crate) fn bhomz_error(msg: impl ToString) -> BhomzError {
		BhomzError::msg(msg.to_string())
	}
}
