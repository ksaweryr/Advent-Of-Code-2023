use std::ops::Mul;

use crate::fraction::Fraction;
use crate::vector::Vector;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Matrix(pub [[Fraction; 3]; 3]);

impl Matrix {
    fn det(&self) -> Fraction {
        let mat = self.0;

        mat[0][0] * (mat[1][1] * mat[2][2] - mat[2][1] * mat[1][2])
        - mat[0][1] * (mat[1][0] * mat[2][2] - mat[2][0] * mat[1][2])
        + mat[0][2] * (mat[1][0] * mat[2][1] - mat[2][0] * mat[1][1])
    }

    fn transpose(&self) -> Matrix {
        let mat = self.0;

        Matrix([
            [mat[0][0], mat[1][0], mat[2][0]],
            [mat[0][1], mat[1][1], mat[2][1]],
            [mat[0][2], mat[1][2], mat[2][2]]
        ])
    }

    fn adjoint(&self) -> Matrix {
        let mat = self.0;

        Matrix([
            [mat[1][1] * mat[2][2] - mat[2][1] * mat[1][2], -(mat[1][0] * mat[2][2] - mat[2][0] * mat[1][2]), mat[1][0] * mat[2][1] - mat[2][0] * mat[1][1]],
            [-(mat[0][1] * mat[2][2] - mat[2][1] * mat[0][2]), mat[0][0] * mat[2][2] - mat[2][0] * mat[0][2], -(mat[0][0] * mat[2][1] - mat[2][0] * mat[0][1])],
            [mat[0][1] * mat[1][2] - mat[1][1] * mat[0][2], -(mat[0][0] * mat[1][2] - mat[1][0] * mat[0][2]), mat[0][0] * mat[1][1] - mat[1][0] * mat[0][1]]
        ]).transpose()
    }

    pub fn inv(&self) -> Option<Matrix> {
        let det = self.det();

        if det == 0.into() {
            None
        } else {
            Some(det.inv() * self.adjoint())
        }
    }
}

impl Mul<Matrix> for Fraction {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix(rhs.0.into_iter()
            .map(|row| row.into_iter()
                .map(|x| self * x)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap())
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(self.0.into_iter()
            .map(|row| row.into_iter().zip(rhs.0.into_iter()).map(|(a, b)| a * b).sum())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::fraction::Fraction;
    use crate::matrix::*;
    use crate::vector::*;

    #[test]
    fn test_det() {
        let m = Matrix([[4.into(), 2.into(), 1.into()], [9.into(), 3.into(), 1.into()], [16.into(), 4.into(), 1.into()]]);

        assert_eq!(m.det(), (-2).into());
    }

    #[test]
    fn test_inv() {
        let m = Matrix([[4.into(), 2.into(), 1.into()], [9.into(), 3.into(), 1.into()], [16.into(), 4.into(), 1.into()]]);
        let expected = Matrix([
            [Fraction::new(1, 2), (-1).into(), Fraction::new(1, 2)],
            [Fraction::new(-7, 2), 6.into(), Fraction::new(-5, 2)],
            [6.into(), (-8).into(), 3.into()]
        ]);

        assert_eq!(m.inv(), Some(expected));
    }

    #[test]
    fn test_mul() {
        let m = Matrix([[1.into(), 2.into(), 3.into()], [4.into(), 5.into(), 6.into()], [7.into(), 8.into(), 9.into()]]);
        let v = Vector([10.into(), 11.into(), 12.into()]);

        assert_eq!(m * v, Vector([68.into(), 167.into(), 266.into()]));
    }
}