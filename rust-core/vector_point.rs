use core::{
    cmp::Ordering,
    f64::consts::PI,
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Rem, RemAssign, Sub},
};
use libm::{atan, fabs, fmod, log, pow, sin, sinh, sqrt};
use serde::{Deserialize, Serialize};

/// Importing necessary types (equivalent to importing from 'values')
use crate::*;

/// A Vector Point uses a structure for 2D or 3D points
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VectorPoint<M: MValueCompatible = MValue> {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate or "altitude". May be None
    pub z: Option<f64>,
    /// M-Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<M>,
    /// T for tolerance. A tmp value used for simplification
    #[serde(skip)]
    pub t: Option<f64>,
}
impl VectorPoint<MValue> {
    /// Helper function for tests. Create a new point quickly from an xy coordinate
    pub fn from_xy(x: f64, y: f64) -> Self {
        Self { x, y, z: None, m: None, t: None }
    }

    /// Helper function for tests. Create a new point quickly from an xyz coordinate
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z: Some(z), m: None, t: None }
    }
}
impl<M: MValueCompatible> VectorPoint<M> {
    /// Create a new point
    pub fn new(x: f64, y: f64, z: Option<f64>, m: Option<M>) -> Self {
        Self { x, y, z, m, t: None }
    }

    /// Project the point into the 0->1 coordinate system
    pub fn project(&mut self, bbox: Option<&mut BBox3D>) {
        let y = self.y;
        let x = self.x;
        let sin = sin((y * PI) / 180.);
        let y2 = 0.5 - (0.25 * log((1. + sin) / (1. - sin))) / PI;
        self.x = x / 360. + 0.5;
        self.y = y2.clamp(0., 1.);

        if let Some(bbox) = bbox {
            bbox.extend_from_point(self)
        };
    }

    /// Unproject the point from the 0->1 coordinate system back to a lon-lat coordinate
    pub fn unproject(&mut self) {
        let lon = (self.x - 0.5) * 360.;
        let y2 = 0.5 - self.y;
        let lat = atan(sinh(PI * (y2 * 2.))).to_degrees();

        self.x = lon;
        self.y = lat;
    }

    /// Calculate the distance between two points, allowing different M types
    pub fn distance<M2: MValueCompatible>(&self, other: &VectorPoint<M2>) -> f64 {
        sqrt(pow(other.x - self.x, 2.) + pow(other.y - self.y, 2.))
    }

    /// Apply modular arithmetic to x, y, and z using `modulus`
    pub fn modulo(self, modulus: f64) -> Self {
        let modulus = fabs(modulus); // Ensure positive modulus
        Self {
            x: fmod(self.x, modulus),
            y: fmod(self.y, modulus),
            z: self.z.map(|z| fmod(z, modulus)),
            m: self.m,
            t: None,
        }
    }
}
impl<M: MValueCompatible> From<Point> for VectorPoint<M> {
    fn from(p: Point) -> Self {
        Self { x: p.0, y: p.1, z: None, m: None, t: None }
    }
}
impl<M: MValueCompatible> From<&Point> for VectorPoint<M> {
    fn from(p: &Point) -> Self {
        Self { x: p.0, y: p.1, z: None, m: None, t: None }
    }
}
impl<M: MValueCompatible> From<Point3D> for VectorPoint<M> {
    fn from(p: Point3D) -> Self {
        Self { x: p.0, y: p.1, z: Some(p.2), m: None, t: None }
    }
}
impl<M: MValueCompatible> From<&Point3D> for VectorPoint<M> {
    fn from(p: &Point3D) -> Self {
        Self { x: p.0, y: p.1, z: Some(p.2), m: None, t: None }
    }
}
impl<M1: MValueCompatible, M2: MValueCompatible> Add<VectorPoint<M2>> for VectorPoint<M1> {
    type Output = VectorPoint<M1>;
    fn add(self, other: VectorPoint<M2>) -> Self::Output {
        VectorPoint {
            x: self.x + other.x,
            y: self.y + other.y,
            // Only add `z` if both `self.z` and `other.z` are `Some`
            z: match (self.z, other.z) {
                (Some(z1), Some(z2)) => Some(z1 + z2),
                _ => None, // If either `z` is None, the result is `None`
            },
            m: self.m.clone(),     // Combine m values
            t: self.t.or(other.t), // Handle `t` as necessary
        }
    }
}
impl<M: MValueCompatible> Add<f64> for VectorPoint<M> {
    type Output = VectorPoint<M>;
    fn add(self, other: f64) -> Self::Output {
        VectorPoint {
            x: self.x + other,
            y: self.y + other,
            z: self.z.map(|z| z + other),
            m: self.m.clone(),
            t: self.t,
        }
    }
}
// Implementing the Sub trait for VectorPoint
impl<M1: MValueCompatible, M2: MValueCompatible> Sub<VectorPoint<M2>> for VectorPoint<M1> {
    type Output = VectorPoint<M1>;
    fn sub(self, other: VectorPoint<M2>) -> Self::Output {
        VectorPoint {
            x: self.x - other.x,
            y: self.y - other.y,
            z: match (self.z, other.z) {
                (Some(z1), Some(z2)) => Some(z1 - z2),
                _ => None, // If either `z` is None, the result is `None`
            },
            m: self.m.clone(),     // Combine m values
            t: self.t.or(other.t), // Handle `t` as necessary
        }
    }
}
impl<M: MValueCompatible> Sub<f64> for VectorPoint<M> {
    type Output = VectorPoint<M>;
    fn sub(self, other: f64) -> Self::Output {
        VectorPoint {
            x: self.x - other,
            y: self.y - other,
            z: self.z.map(|z| z - other),
            m: self.m.clone(),
            t: self.t,
        }
    }
}
// Implementing the Neg trait for VectorPoint
impl<M: MValueCompatible> Neg for VectorPoint<M> {
    type Output = VectorPoint<M>;
    fn neg(self) -> Self::Output {
        VectorPoint { x: -self.x, y: -self.y, z: self.z.map(|z| -z), m: self.m.clone(), t: self.t }
    }
}
// Implementing the Div trait for VectorPoint
impl<M1: MValueCompatible, M2: MValueCompatible> Div<VectorPoint<M2>> for VectorPoint<M1> {
    type Output = VectorPoint<M1>;
    fn div(self, other: VectorPoint<M2>) -> Self::Output {
        VectorPoint {
            x: self.x / other.x,
            y: self.y / other.y,
            z: match (self.z, other.z) {
                (Some(z1), Some(z2)) => Some(z1 / z2),
                _ => None, // If either `z` is None, the result is `None`
            },
            m: self.m.clone(),     // Combine m values
            t: self.t.or(other.t), // Handle `t` as necessary
        }
    }
}
impl<M: MValueCompatible> Div<f64> for VectorPoint<M> {
    type Output = VectorPoint<M>;
    fn div(self, other: f64) -> Self::Output {
        VectorPoint {
            x: self.x / other,
            y: self.y / other,
            z: self.z.map(|z| z / other),
            m: self.m.clone(),
            t: self.t,
        }
    }
}
// Implementing the Mul trait for VectorPoint
impl<M1: MValueCompatible, M2: MValueCompatible> Mul<VectorPoint<M2>> for VectorPoint<M1> {
    type Output = VectorPoint<M1>;
    fn mul(self, other: VectorPoint<M2>) -> Self::Output {
        VectorPoint {
            x: self.x * other.x,
            y: self.y * other.y,
            z: match (self.z, other.z) {
                (Some(z1), Some(z2)) => Some(z1 * z2),
                _ => None, // If either `z` is None, the result is `None`
            },
            m: self.m.clone(),     // Combine m values
            t: self.t.or(other.t), // Handle `t` as necessary
        }
    }
}
impl<M: MValueCompatible> Mul<f64> for VectorPoint<M> {
    type Output = VectorPoint<M>;
    fn mul(self, other: f64) -> Self::Output {
        VectorPoint {
            x: self.x * other,
            y: self.y * other,
            z: self.z.map(|z| z * other),
            m: self.m.clone(),
            t: self.t,
        }
    }
}
impl<M: MValueCompatible> Rem<f64> for VectorPoint<M> {
    type Output = VectorPoint<M>;

    fn rem(self, modulus: f64) -> Self::Output {
        self.modulo(modulus)
    }
}
impl<M: MValueCompatible> RemAssign<f64> for VectorPoint<M> {
    fn rem_assign(&mut self, modulus: f64) {
        let modulus = fabs(modulus);
        self.x = fmod(self.x, modulus);
        self.y = fmod(self.y, modulus);
        if let Some(z) = self.z {
            self.z = Some(fmod(z, modulus));
        }
    }
}
impl<M: MValueCompatible> Eq for VectorPoint<M> {}
impl<M: MValueCompatible> Ord for VectorPoint<M> {
    fn cmp(&self, other: &VectorPoint<M>) -> Ordering {
        match self.x.partial_cmp(&other.x) {
            Some(Ordering::Equal) => {}
            other => return other.unwrap_or(Ordering::Greater), /* Handle cases where `x` comparison is not equal */
        }
        match self.y.partial_cmp(&other.y) {
            Some(Ordering::Equal) => {}
            other => return other.unwrap_or(Ordering::Greater), /* Handle cases where `y` comparison is not equal */
        }
        match self.z.partial_cmp(&other.z) {
            Some(order) => order,
            None => Ordering::Equal, // This handles the NaN case safely
        }
    }
}
impl<M: MValueCompatible> PartialEq for VectorPoint<M> {
    fn eq(&self, other: &VectorPoint<M>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl<M: MValueCompatible> PartialOrd for VectorPoint<M> {
    fn partial_cmp(&self, other: &VectorPoint<M>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let vector_point: VectorPoint = VectorPoint::new(1.0, 2.0, None, None);
        assert_eq!(vector_point.x, 1.0);
        assert_eq!(vector_point.y, 2.0);
        assert_eq!(vector_point.z, None);
        assert_eq!(vector_point.m, None);
        assert_eq!(vector_point.t, None);
    }

    #[test]
    fn from_xy() {
        let vector_point: VectorPoint = VectorPoint::from_xy(1.0, 2.0);
        assert_eq!(vector_point.x, 1.0);
        assert_eq!(vector_point.y, 2.0);
        assert_eq!(vector_point.z, None);
        assert_eq!(vector_point.m, None);
        assert_eq!(vector_point.t, None);
    }

    #[test]
    fn from_xyz() {
        let vector_point: VectorPoint = VectorPoint::from_xyz(1.0, 2.0, 3.0);
        assert_eq!(vector_point.x, 1.0);
        assert_eq!(vector_point.y, 2.0);
        assert_eq!(vector_point.z, Some(3.0));
        assert_eq!(vector_point.m, None);
        assert_eq!(vector_point.t, None);
    }

    #[test]
    fn project() {
        let mut vector_point: VectorPoint = VectorPoint::new(1.0, 2.0, Some(-3.), None);
        let mut bbox: BBox3D = BBox3D::new(1., 1., 0., 0., 0., 1.);
        vector_point.project(Some(&mut bbox));
        assert_eq!(vector_point.x, 0.5027777777777778);
        assert_eq!(vector_point.y, 0.4944433158879836);
        assert_eq!(vector_point.z, Some(-3.));
        assert_eq!(vector_point.m, None);
        assert_eq!(vector_point.t, None);

        assert_eq!(bbox.left, 0.5027777777777778);
        assert_eq!(bbox.bottom, 0.4944433158879836);
        assert_eq!(bbox.right, 0.5027777777777778);
        assert_eq!(bbox.top, 0.4944433158879836);
        assert_eq!(bbox.near, -3.);
        assert_eq!(bbox.far, 1.0);
    }

    #[test]
    fn unproject() {
        let mut vector_point: VectorPoint =
            VectorPoint::new(0.5027777777777778, 0.4944433158879836, Some(-3.), None);
        vector_point.unproject();

        assert_eq!(vector_point.x, 0.9999999999999964);
        assert_eq!(vector_point.y, 2.0000000000000093);
        assert_eq!(vector_point.z, Some(-3.));
        assert_eq!(vector_point.m, None);
        assert_eq!(vector_point.t, None);
    }

    #[test]
    fn test_distance() {
        let vector_point: VectorPoint = VectorPoint::new(1.0, 2.0, None, None);
        let other: VectorPoint = VectorPoint::new(3.0, 4.0, None, None);
        assert_eq!(vector_point.distance(&other), 2.8284271247461903);
    }

    #[test]
    fn from_point() {
        let point = Point(1.0, 2.0);
        let vector_point: VectorPoint = point.into();
        assert_eq!(vector_point.x, 1.0);
        assert_eq!(vector_point.y, 2.0);
        assert_eq!(vector_point.z, None);
        assert_eq!(vector_point.m, None);
        assert_eq!(vector_point.t, None);

        let point = Point(1.0, 2.0);
        let vector_point: VectorPoint = (&point).into();
        assert_eq!(vector_point.x, 1.0);
        assert_eq!(vector_point.y, 2.0);
        assert_eq!(vector_point.z, None);
        assert_eq!(vector_point.m, None);
        assert_eq!(vector_point.t, None);
    }

    #[test]
    fn from_point_3d() {
        let point: Point3D = Point3D(1.0, 2.0, 3.0);
        let vector_point: VectorPoint = point.into();
        assert_eq!(vector_point.x, 1.0);
        assert_eq!(vector_point.y, 2.0);
        assert_eq!(vector_point.z, Some(3.0));
        assert_eq!(vector_point.m, None);
        assert_eq!(vector_point.t, None);

        let point: Point3D = Point3D(1.0, 2.0, 3.0);
        let vector_point: VectorPoint = (&point).into();
        assert_eq!(vector_point.x, 1.0);
        assert_eq!(vector_point.y, 2.0);
        assert_eq!(vector_point.z, Some(3.0));
        assert_eq!(vector_point.m, None);
        assert_eq!(vector_point.t, None);
    }

    #[test]
    fn vector_point() {
        let vector_point: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        assert_eq!(vector_point.x, 1.0);
        assert_eq!(vector_point.y, 2.0);
        assert_eq!(vector_point.z, Some(3.0));
        assert_eq!(vector_point.m, None);
        assert_eq!(vector_point.t, None);
    }

    #[test]
    fn vector_neg() {
        let vector_point = VectorPoint::<MValue> { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let result = -vector_point;
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, Some(-3.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);
    }

    #[test]
    fn vector_point_add() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let vector_point2: VectorPoint =
            VectorPoint { x: 4.0, y: 5.0, z: Some(6.0), m: None, t: Some(5.2) };
        let result = vector_point1 + vector_point2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, Some(9.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, Some(5.2));
    }

    #[test]
    fn vector_point_add_f64() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let float: f64 = 4.0;
        let result = vector_point1 + float;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, Some(7.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);
    }

    #[test]
    fn vector_point_sub() {
        let vector_point1 =
            VectorPoint::<MValue> { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let vector_point2 =
            VectorPoint::<MValue> { x: 4.0, y: 5.0, z: Some(6.0), m: None, t: Some(5.2) };
        let result = vector_point1 - vector_point2;
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, -3.0);
        assert_eq!(result.z, Some(-3.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, Some(5.2));
    }

    #[test]
    fn vector_point_sub_f64() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let float: f64 = 4.0;
        let result = vector_point1 - float;
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, Some(-1.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);
    }

    #[test]
    fn vector_point_mul() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let vector_point2: VectorPoint =
            VectorPoint { x: 4.0, y: 5.0, z: Some(6.0), m: None, t: Some(5.2) };
        let result = vector_point1 * vector_point2;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 10.0);
        assert_eq!(result.z, Some(18.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, Some(5.2));
    }

    #[test]
    fn vector_point_mul_f64() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let float: f64 = 4.0;
        let result = vector_point1 * float;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 8.0);
        assert_eq!(result.z, Some(12.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);
    }

    #[test]
    fn vector_point_div() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let vector_point2: VectorPoint =
            VectorPoint { x: 4.0, y: 5.0, z: Some(6.0), m: None, t: Some(5.2) };
        let result = vector_point1 / vector_point2;
        assert_eq!(result.x, 0.25);
        assert_eq!(result.y, 0.4);
        assert_eq!(result.z, Some(0.5));
        assert_eq!(result.m, None);
        assert_eq!(result.t, Some(5.2));
    }

    #[test]
    fn vector_point_div_f64() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let float: f64 = 4.0;
        let result = vector_point1 / float;
        assert_eq!(result.x, 0.25);
        assert_eq!(result.y, 0.5);
        assert_eq!(result.z, Some(0.75));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);
    }

    #[test]
    fn vector_point_rem() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let result = vector_point1 % 2.;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, Some(1.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);
    }

    #[test]
    fn vector_point_rem_assigned() {
        let mut vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        vector_point1 %= 2.;
        assert_eq!(vector_point1.x, 1.0);
        assert_eq!(vector_point1.y, 0.0);
        assert_eq!(vector_point1.z, Some(1.0));
        assert_eq!(vector_point1.m, None);
        assert_eq!(vector_point1.t, None);
    }

    #[test]
    fn vector_equality() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let vector_point2: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        assert_eq!(vector_point1, vector_point2);

        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let vector_point2: VectorPoint =
            VectorPoint { x: 2.0, y: 3.0, z: Some(4.0), m: None, t: None };
        assert_ne!(vector_point1, vector_point2);

        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let vector_point2: VectorPoint = VectorPoint { x: 1.0, y: 2.0, z: None, m: None, t: None };
        assert_ne!(vector_point1, vector_point2);

        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let vector_point2: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(1.0), m: None, t: None };
        assert_ne!(vector_point1, vector_point2);
    }

    #[test]
    fn test_vectorpoint_ordering_x() {
        let a: VectorPoint = VectorPoint { x: 1.0, y: 0.0, z: None, m: None, t: None };
        let b: VectorPoint = VectorPoint { x: 2.0, y: 0.0, z: None, m: None, t: None };
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }

    #[test]
    fn test_vectorpoint_ordering_y() {
        let a: VectorPoint = VectorPoint { x: 1.0, y: 1.0, z: None, m: None, t: None };
        let b = VectorPoint { x: 1.0, y: 2.0, z: None, m: None, t: None };
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }

    #[test]
    fn test_vectorpoint_ordering_z() {
        let a: VectorPoint = VectorPoint { x: 1.0, y: 1.0, z: Some(1.0), m: None, t: None };
        let b = VectorPoint { x: 1.0, y: 1.0, z: Some(2.0), m: None, t: None };
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }

    #[test]
    fn test_vectorpoint_ordering_z_none() {
        let a: VectorPoint = VectorPoint { x: 1.0, y: 1.0, z: None, m: None, t: None };
        let b = VectorPoint { x: 1.0, y: 1.0, z: Some(2.0), m: None, t: None };
        assert_eq!(a.cmp(&b), Ordering::Less); // `None` is treated as equal to any value in `z`
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }

    #[test]
    fn test_vectorpoint_ordering_z_some() {
        let a: VectorPoint = VectorPoint { x: 1.0, y: 1.0, z: Some(-1.0), m: None, t: None };
        let b = VectorPoint { x: 1.0, y: 1.0, z: Some(2.0), m: None, t: None };
        assert_eq!(a.cmp(&b), Ordering::Less); // `None` is treated as equal to any value in `z`
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }

    #[test]
    fn test_vectorpoint_equality() {
        let a: VectorPoint = VectorPoint { x: 1.0, y: 1.0, z: Some(1.0), m: None, t: None };
        let b = VectorPoint { x: 1.0, y: 1.0, z: Some(1.0), m: None, t: None };
        assert_eq!(a, b);
        assert_eq!(a.cmp(&b), Ordering::Equal);
    }

    #[test]
    fn test_vectorpoint_nan_handling() {
        let nan_point: VectorPoint = VectorPoint { x: f64::NAN, y: 1.0, z: None, m: None, t: None };
        let normal_point = VectorPoint { x: 1.0, y: 1.0, z: None, m: None, t: None };

        // Since `partial_cmp` should return `None` for NaN, `cmp` must not panic.
        assert_eq!(nan_point.cmp(&normal_point), Ordering::Greater);

        // z nan
        let nan_point: VectorPoint =
            VectorPoint { x: 1.0, y: 1.0, z: Some(f64::NAN), m: None, t: None };
        let normal_point = VectorPoint { x: 1.0, y: 1.0, z: Some(1.0), m: None, t: None };
        assert_eq!(nan_point.cmp(&normal_point), Ordering::Equal);
    }

    #[test]
    fn test_vectorpoint_partial_comp() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let vector_point2 = VectorPoint { x: 1.0, y: 2.0, z: Some(1.0), m: None, t: None };

        assert_eq!(vector_point1.partial_cmp(&vector_point2), Some(Ordering::Greater));
        assert_eq!(vector_point2.partial_cmp(&vector_point1), Some(Ordering::Less));
    }

    #[test]
    fn test_vectorpoint_with_m() {
        let vector_point1: VectorPoint<MValue> = VectorPoint {
            x: 1.0,
            y: 2.0,
            z: Some(3.0),
            m: Some(Value::from([
                ("class".into(), ValueType::Primitive(PrimitiveValue::String("ocean".into()))),
                ("offset".into(), ValueType::Primitive(PrimitiveValue::U64(22))),
                (
                    "info".into(),
                    ValueType::Nested(Value::from([
                        (
                            "name".into(),
                            ValueType::Primitive(PrimitiveValue::String("Pacific Ocean".into())),
                        ),
                        ("value".into(), ValueType::Primitive(PrimitiveValue::F32(22.2))),
                    ])),
                ),
            ])),
            t: Some(-1.2),
        };
        let vector_point2: VectorPoint<MValue> =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        assert_eq!(vector_point1.partial_cmp(&vector_point2), Some(Ordering::Equal));
        // we only check for equality in x, y, z
        assert!(vector_point1 == vector_point2);
    }
}
