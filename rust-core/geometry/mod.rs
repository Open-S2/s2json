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
