use std::ops::{Add, Div, Mul, Sub};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Position<T>
where
	T: Clone + Sub + Add + Mul + Div,
{
	pub x: T,
	pub y: T,
}
