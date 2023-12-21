use std::{ops::{Add, Mul, Div, Neg, Sub}, fmt::{Debug, Display}, iter::Sum};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fraction {
    pub p: i128,
    pub q: i128
}

impl Fraction {
    pub fn new(p: i128, q: i128) -> Self {
        let mut result = Fraction { p, q };
        result.simplify();

        result
    }

    pub fn inv(&self) -> Fraction {
        Fraction::new(self.q, self.p)
    }

    fn simplify(&mut self) {
        let sign = (self.p * self.q).signum();
        let m = gcd(self.p.abs(), self.q.abs());
        self.p = sign * self.p.abs() / m;
        self.q = self.q.abs() / m;
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.p, self.q))
    }
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let q = lcm(self.q, rhs.q);
        let p = self.p * (q / self.q) + rhs.p * (q / rhs.q);

        Self::new(p, q)
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let p = self.p * rhs.p;
        let q = self.q * rhs.q;

        Self::new(p, q)
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * Self::new(rhs.q, rhs.p)
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Fraction { p: -self.p, q: self.q }
    }
}

impl Sum for Fraction {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Fraction::new(0, 1), |acc, f| acc + f)
    }
}

impl From<i128> for Fraction {
    fn from(value: i128) -> Self {
        Self::new(value, 1)
    }
}

fn gcd(a: i128, b: i128) -> i128 {
    let (a, b) = if a < b { (b, a) } else { (a, b) };

    if b == 0 {
        a
    } else {
        gcd(b, a.rem_euclid(b))
    }
}

fn lcm(a: i128, b: i128) -> i128 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use crate::fraction::*;

    #[test]
    fn test_add() {
        let f1 = Fraction::new(2, 3);
        let f2 = Fraction::new(3, 4);

        assert_eq!(f1 + f2, Fraction::new(17, 12));
    }

    #[test]
    fn test_mul() {
        let f1 = Fraction::new(1, 6);
        let f2 = Fraction::new(3, 14);

        assert_eq!(f1 * f2, Fraction::new(1, 28));
    }

    #[test]
    fn test_div() {
        let f1 = Fraction::new(1, 6);
        let f2 = Fraction::new(3, 2);

        assert_eq!(f1 / f2, Fraction::new(1, 9));
    }

    #[test]
    fn test_sum() {
        let fs = [Fraction::new(1, 3), Fraction::new(3, 4), Fraction::new(5, 2)];

        assert_eq!(fs.into_iter().sum::<Fraction>(), Fraction::new(43, 12));
    }
}