use std::ops::{Mul, Add};
use std::process::Output;

struct Scalar { value: f64 }

struct Vector { value: Vec<f64> }

struct UnitVectors {
    x: Vector,
    y: Vector,
    z: Vector,
    w: Some(Vector)
}

const UNIT_VECTORS_3D: UnitVectors = UnitVectors {
    x: Vector { value: vec![1.0, 0.0, 0.0]},
    y: Vector { value: vec![0.0, 1.0, 0.0]},
    z: Vector { value: vec![0.0, 0.0, 1.0]},
    w: Some(()),
};

const UNIT_VECTORS_4D: UnitVectors = UnitVectors {
    x: Vector { value: vec![1.0, 0.0, 0.0, 0.0] },
    y: Vector { value: vec![0.0, 1.0, 0.0, 0.0] },
    z: Vector { value: vec![0.0, 0.0, 1.0, 0.0] },
    w: Some( Vector { value: vec![0.0, 0.0, 0.0, 1.0] } )
};

pub trait Mag {
    type Output = f64;

    fn mag(&self) -> Self::Output {
        squared_mag(&self).sqrt()
    }
    fn squared_mag(&self) -> Self::Output;
}
/// A trait that defines the function used for vector dot products.
/// # Arguments
/// - `self` a vector type of n-dimension where `n`<sub>`self`</sub>` = n`<sub>`other`</sub>
/// - `other` a vector type of n-dimension where n<sub>other</sub> = n<sub>self</sub>
///
pub trait Dot {
    type Output = f64;

    fn dot (&self, other: &Vector) -> Self::Output;
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
    type Output = Vector;

    fn cross (&self, other: &Vector) -> Output;
}

impl Cross for Vector  {
    fn cross(&self, other: &Vector) -> Some(Vector) {
        // assert_eq!(self.value.len(), 3); // must be 3-dimensional
        // assert_eq!(other.value.len(), 3); // must be 3-dimensional

        if &self.value.len() != 3 || other.value.len() != 3 {
            return None;
        }

        let a = &self.value;
        let b = &other.value;

        let cross_x = (a.y * b.z) - (a.z * b.y);
        let cross_y = (a.z * b.x) - (a.x * b.z);
        let cross_z = (a.x * b.y) - (a.y * b.z);

        return Some(Vector { value: vec![cross_x, cross_y, cross_z] })
    }
}

impl Dot for Vector {
    fn dot(&self, other: &Vector) -> usize {
        assert_eq!(self.value.len(), other.value.len()); // must be same size

        let zip = Iterator::zip(self.value.iter(), other.value.iter());

        let mut total: usize = 0;
        for (a, b) in zip {
            total += a * b;
        }

        return total
    }
}

impl Mul<Scalar> for Vector {
    type Output = Self;

    fn mul(&self, rhs: &Scalar) -> Self::Output {
        Self { value: self.value.iter().map(|v| v * rhs.value).collect()}
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(&self, rhs: &Self) -> Self::Output {
        Self { value: Iterator::zip(self.value.iter(), rhs.value.iter()).map(|t| t.0 + t.1).collect() }
    }
}

