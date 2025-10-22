#[derive(Debug)]
pub struct BhomzError {
	msg: String,
}
pub type BhomzResult<T> = Result<T, BhomzError>;
pub type BhomzThrowable = BhomzResult<()>;

#[inline(always)]
pub fn bhomz_error(msg: impl ToString) -> BhomzError {
	BhomzError { msg: msg.to_string() }
}

#[inline(always)]
pub fn bhomz_error_borrow(msg: &impl ToString) -> BhomzError {
	BhomzError { msg: msg.to_string() }
}