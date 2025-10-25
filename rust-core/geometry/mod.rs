/// BBox and BBox3D shapes and utilities
pub mod bbox;
/// Impls that we want to hide to make the code more readable
pub mod impls;
/// Primitive geometry types (used by GeoJSON spec)
pub mod primitive;
/// Vector geometry types (used by the s2json spec for both WGS84 and S2Geometry)
pub mod vector;
/// The VectorPoint struct is a powerful tool for 2D and 3D points
pub mod vector_point;

use crate::Face;
pub use bbox::*;
pub use primitive::*;
use serde::{Deserialize, Serialize};
pub use vector::*;
pub use vector_point::*;

/// Trait to extract the x and y values
pub trait GetXY {
    /// Returns the x value
    fn x(&self) -> f64;
    /// Returns the y value
    fn y(&self) -> f64;
}
/// Trait to extract the z value
pub trait GetZ {
    /// Returns the z value
    fn z(&self) -> Option<f64>;
}
/// Trait to extract the m value
pub trait GetM<M> {
    /// Returns the m value
    fn m(&self) -> Option<&M>;
}

/// Composite Trait: XY + Z
pub trait GetXYZ: GetXY + GetZ {}
/// Composite Trait: XY + M
pub trait GetXYM<M>: GetXY + GetM<M> {}
/// Composite Trait: XY + Z + M
pub trait GetXYZM<M>: GetXY + GetZ + GetM<M> {}

/// Trait to set the x and y values
pub trait SetXY {
    /// Set the x value
    fn set_x(&mut self, x: f64);
    /// Set the y value
    fn set_y(&mut self, y: f64);
    /// Set both x and y
    fn set_xy(&mut self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }
}
/// Trait to set the z value
pub trait SetZ {
    /// Set the z value
    fn set_z(&mut self, z: f64);
}
/// Trait to set the m value
pub trait SetM<M> {
    /// Set the m value
    fn set_m(&mut self, m: M);
}

/// Composite Trait: XY + Z
pub trait SetXYZ: SetXY + SetZ {
    /// Set x, y and z
    fn set_xyz(&mut self, x: f64, y: f64, z: f64) {
        self.set_xy(x, y);
        self.set_z(z);
    }
}
/// Composite Trait: XY + M
pub trait SetXYM<M>: SetXY + SetM<M> {
    /// Set x, y and m
    fn set_xym(&mut self, x: f64, y: f64, m: M) {
        self.set_xy(x, y);
        self.set_m(m);
    }
}
/// Composite Trait: XY + Z + M
pub trait SetXYZM<M>: SetXY + SetZ + SetM<M> {
    /// Set x, y, z and m
    fn set_xyzm(&mut self, x: f64, y: f64, z: f64, m: M) {
        self.set_xy(x, y);
        self.set_z(z);
        self.set_m(m);
    }
}

/// Trait to create a new XY
pub trait NewXY {
    /// Create a new point with xy
    fn new_xy(x: f64, y: f64) -> Self;
}
/// Trait to create a new XYZ
pub trait NewXYZ {
    /// Create a new point with xyz
    fn new_xyz(x: f64, y: f64, z: f64) -> Self;
}
/// Trait to create a new XYZM
pub trait NewXYZM<M> {
    /// Create a new point with xyzm
    fn new_xyzm(x: f64, y: f64, z: f64, m: M) -> Self;
}

/// The axis to apply an operation to
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    /// X axis
    X = 0,
    /// Y axis
    Y = 1,
}

/// A Point in S2 Space with a Face
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct STPoint<M> {
    /// The face of the point
    pub face: Face,
    /// The s coordinate
    pub s: f64,
    /// The t coordinate
    pub t: f64,
    /// The z coordinate
    pub z: Option<f64>,
    /// The m coordinate
    pub m: Option<M>,
}
