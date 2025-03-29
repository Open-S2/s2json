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

use crate::{Face, MValue};
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
    fn z(&self) -> f64;
}
/// Trait to extract the m value
pub trait GetM<M = MValue> {
    /// Returns the m value
    fn m(&self) -> Option<&M>;
}

/// Composite Trait: XY + Z
pub trait GetXYZ: GetXY + GetZ {}
/// Composite Trait: XY + M
pub trait GetXYM<M = MValue>: GetXY + GetM<M> {}
/// Composite Trait: XY + Z + M
pub trait GetXYZM<M = MValue>: GetXY + GetZ + GetM<M> {}

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
pub struct STPoint<M = MValue> {
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
