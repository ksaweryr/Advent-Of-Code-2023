use crate::fraction::Fraction;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector(pub [Fraction; 3]);