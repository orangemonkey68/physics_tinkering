extern crate num_traits;

use std::ops::{Mul, Add, Sub, AddAssign};
use std::process::Output;
use num_traits::Num;

#[derive(Debug, Copy, Clone)]
pub struct Scalar { value: f64 }


//TODO: Re-implement with generics (Vector<T> or Vector<T, S>)
#[derive(Debug, Clone)]
pub struct Vector {
    value: Vec<f64>
}

impl Vector {
    pub fn new(value: Vec<i32>) -> Self {
        Vector {value}
    }
}

impl Scalar {
    pub fn new(value: f64) -> Self {
        Scalar{value}
    }
}

struct UnitVectors {
    x: Vector,
    y: Vector,
    z: Vector,
    w: Option<Vector>
}

pub const UNIT_VECTORS_3D: UnitVectors = UnitVectors {
    x: Vector { value: vec![1.0, 0.0, 0.0]},
    y: Vector { value: vec![0.0, 1.0, 0.0]},
    z: Vector { value: vec![0.0, 0.0, 1.0]},
    w: None,
};

pub const UNIT_VECTORS_4D: UnitVectors = UnitVectors {
    x: Vector { value: vec![1.0, 0.0, 0.0, 0.0] },
    y: Vector { value: vec![0.0, 1.0, 0.0, 0.0] },
    z: Vector { value: vec![0.0, 0.0, 1.0, 0.0] },
    w: Some( Vector { value: vec![0.0, 0.0, 0.0, 1.0] } )
};

pub trait Mag {
    fn mag(&self) -> f64 {
        self.squared_mag().sqrt()
    }
    fn squared_mag(&self) -> f64;
}
/// A trait that defines the function used for vector dot products.
/// # Arguments
/// - `self` a vector type of n-dimension where `n`<sub>`self`</sub>` = n`<sub>`other`</sub>
/// - `other` a vector type of n-dimension where n<sub>other</sub> = n<sub>self</sub>
///
pub trait Dot {
    fn dot (&self, other: &Vector) -> f64;
}

/// A trait that defines the function used for vector cross products.
///
/// # Arguments
///
/// * `self` - A Vector type
/// * `other` - Another Vector type
/// ## NOTE: Both vectors MUST be EXACTLY 3-dimensional.
///
/// # Examples
/// ```
/// use physics_tinkering::math;
///
/// pub fn main() {
///     let v1 = Vector::new(vec![1, 0, 0])
///     let v2 = Vector::new(vec![0, 1, 0])
///     assert_eq(Vector::new(vec![0, 0, 1]), v1.cross(&v2))
/// ```
/// }
pub trait Cross {
    fn cross (&self, other: &Vector) -> Vector;
}

impl Cross for Vector  {
    fn cross(self, other: &Vector) -> Option<Vector> {
        // assert_eq!(self.value.len(), 3); // must be 3-dimensional
        // assert_eq!(other.value.len(), 3); // must be 3-dimensional

        if self.value.len() != 3 || other.value.len() != 3 {
            return None;
        }

        let a = self.value;
        let b = &other.value;

        let cross_x = (a[1] * b[2]) - (a[2] * b[1]);
        let cross_y = (a[2] * b[0]) - (a[0] * b[2]);
        let cross_z = (a[0] * b[1]) - (a[1] * b[2]);

        return Some(Vector { value: vec![cross_x, cross_y, cross_z] })
    }
}

impl Dot for Vector {
    fn dot(&self, other: &Vector) -> f64 {
        assert_eq!(self.value.len(), other.value.len()); // must be same size

        let zip = Iterator::zip(self.value.iter(), other.value.iter());

        let mut total: f64 = 0.0;
        for (a, b) in zip {
            total += a * b;
        }

        return total
    }
}

impl Mul<Scalar> for Vector {
    type Output = Self;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        Self { value: self.value.iter().map(|v| v * rhs.value).collect()}
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self { value: Iterator::zip(self.value.iter(), rhs.value.iter()).map(|t| t.0 + t.1).collect() }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        let vec = &self.value;
        let other_vec = &rhs.value;

        Self { value: Iterator::zip(vec.iter(), other_vec.iter()).map(|t| t.0 - t.1).collect()}
    }
}

// Matrices

pub struct Matrix<T: Num> {
    value: Vec<Vec<T>>,
}

impl Matrix<T> {
    pub fn new() -> Matrix<T> {Matrix {value: vec![vec![]]}}
    pub fn from_vec(v: Vec<Vec<T>>) -> Matrix<T> {
        Matrix { value: v }
    }

    pub fn is_square(&self) -> bool {
        self.value.len() == self.value[0].len()
    }

    /// Returns TRUE if the matrix is a consistent x by y size, FALSE if otherwise
    ///
    /// # Example
    ///
    /// ```
    ///
    /// // {1, 0, 0}
    /// // {0, 1, 0}
    /// // {0, 0, 1}
    /// let m1 = Matrix::from_vec(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1])
    ///
    /// // {1, 0}
    /// // {0, 1, 0}
    /// // {0, 0, 1}
    /// let m2 = Matrix::from_vec(vec![vec![1, 0], vec![0, 1, 0], vec![0, 0, 1]])
    ///
    /// assert_eq!(m1.is_legal(), true);
    /// assert_eq!(m2.is_legal(), false);
    pub fn is_legal(&self) -> bool {
        let mat = &self.value;
        let row_widths: Vec<i32> = vec![];

        for x in layout {
            for y in x {
                row_widths[x] += 1;
            }
        }

        let has_consistent_width = row_widths == vec![row_widths[0]; row_widths.len()];
    }
}

pub trait Det {
    type Output;

    fn det(&self) -> Self::Output;

    //not intended for API
    fn det_n(&self, n: i32) -> Self::Output;
}

// Sampled from https://www.geeksforgeeks.org/determinant-of-a-matrix/
impl Det for Matrix<T> {
    type Output = T;

    fn det(&self) -> Self::Output {
        self.det_n(&self.value.len() as i32)
    }

    fn det_n(&self, n: i32) -> Self::Output {
        let d: T = 0;
        let mat: &Vec<Vec<T>> = &self.value;

        if n == 1 {
            return mat[0][0]
        }

        let mut cofactors = Matrix::new();
        let sign: T = 1;

        for (i, x) in mat[0].iter().enumerate() {
            get_cofactor(&self, &mut cofactors, 0, i, n);
            d += sign * mat[0][i] * &self.det_n(cofactors, n-1)
        }
    }
}

// helper func

pub fn get_cofactor<T: Num>(mat: &Matrix<T>, cofactors: &mut Vec<Vec<T>>, x: T, y: T, n: T) {
    let mut i = 0;
    let mut j = 0;

    for row in 0..n {
        for col in 0..n {
            if row != x && col != y {
                cofactors[i][j] = &mat.value[row][col];
                j += 1;

                if j == &n-1 {
                    j = 0;
                    i += 1;
                }
            }
        }
    }
}

