use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

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

#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Dimension<T>
where
	T: Clone + Sub + Add + Mul + Div,
{
	pub x: T,
	pub y: T,
	pub width: T,
	pub height: T,
}
