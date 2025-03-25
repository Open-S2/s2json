use core::{
    cmp::Ordering,
    f64::consts::PI,
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};
use libm::{atan, atan2, fabs, fmod, log, sin, sinh, sqrt};
use serde::{Deserialize, Serialize};

/// Importing necessary types (equivalent to importing from 'values')
use crate::*;

/// A Vector Point uses a structure for 2D or 3D points
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[repr(C)]
pub struct VectorPoint<M: Clone = MValue> {
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
impl<M: Clone> VectorPoint<M> {
    /// Create a new point
    pub fn new(x: f64, y: f64, z: Option<f64>, m: Option<M>) -> Self {
        Self { x, y, z, m, t: None }
    }

    /// Create a new point with xy
    pub fn new_xy(x: f64, y: f64, m: Option<M>) -> Self {
        Self { x, y, z: None, m, t: None }
    }

    /// Create a new point with xyz
    pub fn new_xyz(x: f64, y: f64, z: f64, m: Option<M>) -> Self {
        Self { x, y, z: Some(z), m, t: None }
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

    /// Returns true if the point is the zero vector.
    pub fn is_empty(&self) -> bool {
        let zero = f64::default();
        self.x == zero && self.y == zero && (self.z.is_none() || self.z.unwrap() == zero)
    }

    /// Returns the S2 face assocated with this point
    pub fn face(&self, f: u8) -> f64 {
        if f == 0 {
            self.x
        } else if f == 1 {
            self.y
        } else {
            self.z.unwrap_or(0.)
        }
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

    /// Returns the angle between "this" and v in radians, in the range [0, pi]. If
    /// either vector is zero-length, or nearly zero-length, the result will be
    /// zero, regardless of the other value.
    pub fn angle<M2: Clone>(&self, b: &VectorPoint<M2>) -> f64 {
        atan2(self.cross(b).norm(), self.dot(b))
    }

    /// Get the cross product of two Vector Points
    pub fn cross<M2: Clone>(&self, b: &VectorPoint<M2>) -> Self {
        if let (Some(z), Some(bz)) = (self.z, b.z) {
            Self::new_xyz(
                self.y * bz - z * b.y,
                z * b.x - self.x * bz,
                self.x * b.y - self.y * b.x,
                None,
            )
        } else {
            Self::new_xy(self.x * b.y - self.y * b.x, self.y * b.x - self.x * b.y, None)
        }
    }

    /// dot returns the standard dot product of v and ov.
    pub fn dot<M2: Clone>(&self, b: &VectorPoint<M2>) -> f64 {
        if let (Some(z), Some(bz)) = (self.z, b.z) {
            self.x * b.x + self.y * b.y + z * bz
        } else {
            self.x * b.x + self.y * b.y
        }
    }

    /// Returns the absolute value of the point.
    pub fn abs(&self) -> Self {
        Self::new(fabs(self.x), fabs(self.y), self.z.map(fabs), None)
    }

    /// Returns the inverse of the point
    pub fn invert(&self) -> Self {
        -self
    }

    /// Returns the length of the point.
    pub fn len(&self) -> f64 {
        self.norm()
    }

    /// norm returns the vector's norm.
    pub fn norm(&self) -> f64 {
        sqrt(self.norm2())
    }

    /// norm2 returns the vector's squared norm.
    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    /// Normalize this point to unit length.
    pub fn normalize(&mut self) {
        let len = self.len();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            if let Some(z) = self.z {
                self.z = Some(z / len);
            }
        }
    }

    /// return the distance from this point to the other point in radians
    pub fn distance<M2: Clone>(&self, b: &VectorPoint<M2>) -> f64 {
        let d: VectorPoint<M> = self - b;
        d.len()
    }

    /// Returns the largest absolute component of the point.
    pub fn largest_abs_component(&self) -> u8 {
        let tmp = self.abs();
        let tmp_z = tmp.z.unwrap_or(-2.);
        if tmp.x > tmp.y {
            if tmp.x > tmp_z {
                0
            } else {
                2
            }
        } else if tmp.y > tmp_z {
            1
        } else {
            2
        }
    }

    /// Returns the intermediate point between this and the other point.
    pub fn intermediate<M2: Clone>(&self, b: &VectorPoint<M2>, t: f64) -> Self {
        if let (Some(z), Some(bz)) = (self.z, b.z) {
            Self::new_xyz(
                self.x + ((b.x - self.x) * (1.0 - t)),
                self.y + ((b.y - self.y) * (1.0 - t)),
                z + ((bz - z) * (1.0 - t)),
                None,
            )
        } else {
            Self::new_xy(
                self.x + ((b.x - self.x) * (1.0 - t)),
                self.y + ((b.y - self.y) * (1.0 - t)),
                None,
            )
        }
    }

    /// Returns the perpendicular vector
    pub fn perpendicular(&self) -> Self {
        if let Some(z) = self.z {
            let ref_point = if fabs(self.x) > fabs(z) {
                Self::new_xyz(0., 0., 1., None)
            } else {
                Self::new_xyz(1., 0., 0., None)
            };
            let cross = self.cross(&ref_point);
            Self::new_xyz(-self.y, self.x, cross.z.unwrap(), None)
        } else {
            Self::new_xy(-self.y, self.x, None)
        }
    }
}
impl<M: Clone> From<Point> for VectorPoint<M> {
    fn from(p: Point) -> Self {
        Self { x: p.0, y: p.1, z: None, m: None, t: None }
    }
}
impl<M: Clone> From<&Point> for VectorPoint<M> {
    fn from(p: &Point) -> Self {
        Self { x: p.0, y: p.1, z: None, m: None, t: None }
    }
}
impl<M: Clone> From<Point3D> for VectorPoint<M> {
    fn from(p: Point3D) -> Self {
        Self { x: p.0, y: p.1, z: Some(p.2), m: None, t: None }
    }
}
impl<M: Clone> From<&Point3D> for VectorPoint<M> {
    fn from(p: &Point3D) -> Self {
        Self { x: p.0, y: p.1, z: Some(p.2), m: None, t: None }
    }
}
impl<M1: Clone, M2: Clone> Add<&VectorPoint<M2>> for &VectorPoint<M1> {
    type Output = VectorPoint<M1>;
    fn add(self, other: &VectorPoint<M2>) -> Self::Output {
        VectorPoint {
            x: self.x + other.x,
            y: self.y + other.y,
            // Only add `z` if both `self.z` and `other.z` are `Some`
            z: match (self.z, other.z) {
                (Some(z1), Some(z2)) => Some(z1 + z2),
                _ => None, // If either `z` is None, the result is `None`
            },
            m: None, // Combine m values
            t: None, // Handle `t` as necessary
        }
    }
}
impl<M1: Clone, M2: Clone> AddAssign<&VectorPoint<M2>> for VectorPoint<M1> {
    fn add_assign(&mut self, other: &VectorPoint<M2>) {
        self.x += other.x;
        self.y += other.y;
        if let (Some(z), Some(other_z)) = (self.z, other.z) {
            self.z = Some(z + other_z);
        }
    }
}
impl<M: Clone> Add<f64> for &VectorPoint<M> {
    type Output = VectorPoint<M>;
    fn add(self, other: f64) -> Self::Output {
        VectorPoint {
            x: self.x + other,
            y: self.y + other,
            z: self.z.map(|z| z + other),
            m: None,
            t: None,
        }
    }
}
impl<M: Clone> AddAssign<f64> for VectorPoint<M> {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        if let Some(z) = self.z {
            self.z = Some(z + other);
        }
    }
}
// Implementing the Sub trait for VectorPoint
impl<M1: Clone, M2: Clone> Sub<&VectorPoint<M2>> for &VectorPoint<M1> {
    type Output = VectorPoint<M1>;
    fn sub(self, other: &VectorPoint<M2>) -> Self::Output {
        VectorPoint {
            x: self.x - other.x,
            y: self.y - other.y,
            z: match (self.z, other.z) {
                (Some(z1), Some(z2)) => Some(z1 - z2),
                _ => None, // If either `z` is None, the result is `None`
            },
            m: None, // Combine m values
            t: None, // Handle `t` as necessary
        }
    }
}
impl<M1: Clone, M2: Clone> SubAssign<&VectorPoint<M2>> for VectorPoint<M1> {
    fn sub_assign(&mut self, other: &VectorPoint<M2>) {
        self.x -= other.x;
        self.y -= other.y;
        if let (Some(z), Some(other_z)) = (self.z, other.z) {
            self.z = Some(z - other_z);
        }
    }
}
impl<M: Clone> Sub<f64> for &VectorPoint<M> {
    type Output = VectorPoint<M>;
    fn sub(self, other: f64) -> Self::Output {
        VectorPoint {
            x: self.x - other,
            y: self.y - other,
            z: self.z.map(|z| z - other),
            m: None,
            t: None,
        }
    }
}
impl<M: Clone> SubAssign<f64> for VectorPoint<M> {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
        if let Some(z) = self.z {
            self.z = Some(z - other);
        }
    }
}
// Implementing the Neg trait for VectorPoint
impl<M: Clone> Neg for &VectorPoint<M> {
    type Output = VectorPoint<M>;
    fn neg(self) -> Self::Output {
        VectorPoint { x: -self.x, y: -self.y, z: self.z.map(|z| -z), m: None, t: None }
    }
}
// Implementing the Div trait for VectorPoint
impl<M1: Clone, M2: Clone> Div<&VectorPoint<M2>> for &VectorPoint<M1> {
    type Output = VectorPoint<M1>;
    fn div(self, other: &VectorPoint<M2>) -> Self::Output {
        VectorPoint {
            x: self.x / other.x,
            y: self.y / other.y,
            z: match (self.z, other.z) {
                (Some(z1), Some(z2)) => Some(z1 / z2),
                _ => None, // If either `z` is None, the result is `None`
            },
            m: None, // Combine m values
            t: None, // Handle `t` as necessary
        }
    }
}
impl<M1: Clone, M2: Clone> DivAssign<&VectorPoint<M2>> for VectorPoint<M1> {
    fn div_assign(&mut self, other: &VectorPoint<M2>) {
        self.x /= other.x;
        self.y /= other.y;
        if let (Some(z), Some(other_z)) = (self.z, other.z) {
            self.z = Some(z / other_z);
        }
    }
}
impl<M: Clone> Div<f64> for &VectorPoint<M> {
    type Output = VectorPoint<M>;
    fn div(self, other: f64) -> Self::Output {
        VectorPoint {
            x: self.x / other,
            y: self.y / other,
            z: self.z.map(|z| z / other),
            m: None,
            t: None,
        }
    }
}
impl<M: Clone> DivAssign<f64> for VectorPoint<M> {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        if let Some(z) = self.z {
            self.z = Some(z / other);
        }
    }
}
// Implementing the Mul trait for VectorPoint
impl<M1: Clone, M2: Clone> Mul<&VectorPoint<M2>> for &VectorPoint<M1> {
    type Output = VectorPoint<M1>;
    fn mul(self, other: &VectorPoint<M2>) -> Self::Output {
        VectorPoint {
            x: self.x * other.x,
            y: self.y * other.y,
            z: match (self.z, other.z) {
                (Some(z1), Some(z2)) => Some(z1 * z2),
                _ => None, // If either `z` is None, the result is `None`
            },
            m: None, // Combine m values
            t: None, // Handle `t` as necessary
        }
    }
}
impl<M1: Clone, M2: Clone> MulAssign<&VectorPoint<M2>> for VectorPoint<M1> {
    fn mul_assign(&mut self, other: &VectorPoint<M2>) {
        self.x *= other.x;
        self.y *= other.y;
        if let (Some(z), Some(other_z)) = (self.z, other.z) {
            self.z = Some(z * other_z);
        }
    }
}
impl<M: Clone> Mul<f64> for &VectorPoint<M> {
    type Output = VectorPoint<M>;
    fn mul(self, other: f64) -> Self::Output {
        VectorPoint {
            x: self.x * other,
            y: self.y * other,
            z: self.z.map(|z| z * other),
            m: None,
            t: None,
        }
    }
}
impl<M: Clone> MulAssign<f64> for VectorPoint<M> {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        if let Some(z) = self.z {
            self.z = Some(z * other);
        }
    }
}
impl<M: Clone> Rem<f64> for VectorPoint<M> {
    type Output = VectorPoint<M>;

    fn rem(self, modulus: f64) -> Self::Output {
        self.modulo(modulus)
    }
}
impl<M: Clone> RemAssign<f64> for VectorPoint<M> {
    fn rem_assign(&mut self, modulus: f64) {
        let modulus = fabs(modulus);
        self.x = fmod(self.x, modulus);
        self.y = fmod(self.y, modulus);
        if let Some(z) = self.z {
            self.z = Some(fmod(z, modulus));
        }
    }
}
impl<M: Clone> Eq for VectorPoint<M> {}
impl<M: Clone> Ord for VectorPoint<M> {
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
impl<M: Clone> PartialEq for VectorPoint<M> {
    fn eq(&self, other: &VectorPoint<M>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl<M: Clone> PartialOrd for VectorPoint<M> {
    fn partial_cmp(&self, other: &VectorPoint<M>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
        assert_eq!(size_of::<VectorPoint<()>>(), 56);
        assert_eq!(size_of::<VectorPoint<MValue>>(), 80);
    }

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
    fn new_xy() {
        let vector_point: VectorPoint = VectorPoint::new_xy(1.0, 2.0, None);
        assert_eq!(vector_point.x, 1.0);
        assert_eq!(vector_point.y, 2.0);
        assert_eq!(vector_point.z, None);
        assert_eq!(vector_point.m, None);
        assert_eq!(vector_point.t, None);
    }

    #[test]
    fn new_xyz() {
        let vector_point: VectorPoint = VectorPoint::new_xyz(1.0, 2.0, 3.0, None);
        assert_eq!(vector_point.x, 1.0);
        assert_eq!(vector_point.y, 2.0);
        assert_eq!(vector_point.z, Some(3.0));
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
    fn project_no_bbox() {
        let mut vector_point: VectorPoint = VectorPoint::new(1.0, 2.0, Some(-3.), None);
        vector_point.project(None);
        assert_eq!(vector_point.x, 0.5027777777777778);
        assert_eq!(vector_point.y, 0.4944433158879836);
        assert_eq!(vector_point.z, Some(-3.));
        assert_eq!(vector_point.m, None);
        assert_eq!(vector_point.t, None);
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
        let result = -&vector_point;
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
        let result = &vector_point1 + &vector_point2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, Some(9.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);

        let vector_point1 = VectorPoint::from_xy(1., 2.);
        let vector_point2 = VectorPoint::from_xy(4., 5.);
        let result = &vector_point1 + &vector_point2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, None);
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);

        // assign
        let mut vector_point1 = VectorPoint::from_xyz(1., 2., 3.);
        let vector_point2 = VectorPoint::from_xyz(4., 5., 6.);
        vector_point1 += &vector_point2;
        assert_eq!(vector_point1.x, 5.0);
        assert_eq!(vector_point1.y, 7.0);
        assert_eq!(vector_point1.z, Some(9.0));
        assert_eq!(vector_point1.m, None);
        assert_eq!(vector_point1.t, None);
    }

    #[test]
    fn vector_point_add_f64() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let float: f64 = 4.0;
        let result = &vector_point1 + float;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, Some(7.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);

        // assign
        let mut vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let float: f64 = 4.0;
        vector_point1 += float;
        assert_eq!(vector_point1.x, 5.0);
        assert_eq!(vector_point1.y, 6.0);
        assert_eq!(vector_point1.z, Some(7.0));
        assert_eq!(vector_point1.m, None);
        assert_eq!(vector_point1.t, None);
    }

    #[test]
    fn vector_point_sub() {
        let vector_point1 =
            VectorPoint::<MValue> { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let vector_point2 =
            VectorPoint::<MValue> { x: 4.0, y: 5.0, z: Some(6.0), m: None, t: Some(5.2) };
        let result = &vector_point1 - &vector_point2;
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, -3.0);
        assert_eq!(result.z, Some(-3.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);

        let vector_point1 = VectorPoint::from_xy(1., 2.);
        let vector_point2 = VectorPoint::from_xy(4., 5.);
        let result = &vector_point1 - &vector_point2;
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, -3.0);
        assert_eq!(result.z, None);
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);

        // assign
        let mut vector_point1 = VectorPoint::from_xyz(1., 2., 3.);
        let vector_point2 = VectorPoint::from_xyz(4., 5., 6.);
        vector_point1 -= &vector_point2;
        assert_eq!(vector_point1.x, -3.0);
        assert_eq!(vector_point1.y, -3.0);
        assert_eq!(vector_point1.z, Some(-3.0));
        assert_eq!(vector_point1.m, None);
        assert_eq!(vector_point1.t, None);
    }

    #[test]
    fn vector_point_sub_f64() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let float: f64 = 4.0;
        let result = &vector_point1 - float;
        assert_eq!(result.x, -3.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, Some(-1.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);

        // assign
        let mut vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let float: f64 = 4.0;
        vector_point1 -= float;
        assert_eq!(vector_point1.x, -3.0);
        assert_eq!(vector_point1.y, -2.0);
        assert_eq!(vector_point1.z, Some(-1.0));
        assert_eq!(vector_point1.m, None);
        assert_eq!(vector_point1.t, None);
    }

    #[test]
    fn vector_point_mul() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let vector_point2: VectorPoint =
            VectorPoint { x: 4.0, y: 5.0, z: Some(6.0), m: None, t: Some(5.2) };
        let result = &vector_point1 * &vector_point2;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 10.0);
        assert_eq!(result.z, Some(18.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);

        let vector_point1 = VectorPoint::from_xy(1., 2.);
        let vector_point2 = VectorPoint::from_xy(4., 5.);
        let result = &vector_point1 * &vector_point2;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 10.0);
        assert_eq!(result.z, None);
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);

        // assign
        let mut vector_point1 = VectorPoint::from_xyz(1., 2., 3.);
        let vector_point2 = VectorPoint::from_xyz(4., 5., 6.);
        vector_point1 *= &vector_point2;
        assert_eq!(vector_point1.x, 4.0);
        assert_eq!(vector_point1.y, 10.0);
        assert_eq!(vector_point1.z, Some(18.0));
        assert_eq!(vector_point1.m, None);
        assert_eq!(vector_point1.t, None);
    }

    #[test]
    fn vector_point_mul_f64() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let float: f64 = 4.0;
        let result = &vector_point1 * float;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 8.0);
        assert_eq!(result.z, Some(12.0));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);

        // assign
        let mut vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let float: f64 = 4.0;
        vector_point1 *= float;
        assert_eq!(vector_point1.x, 4.0);
        assert_eq!(vector_point1.y, 8.0);
        assert_eq!(vector_point1.z, Some(12.0));
        assert_eq!(vector_point1.m, None);
        assert_eq!(vector_point1.t, None);
    }

    #[test]
    fn vector_point_div() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let vector_point2: VectorPoint =
            VectorPoint { x: 4.0, y: 5.0, z: Some(6.0), m: None, t: Some(5.2) };
        let result = &vector_point1 / &vector_point2;
        assert_eq!(result.x, 0.25);
        assert_eq!(result.y, 0.4);
        assert_eq!(result.z, Some(0.5));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);

        let vector_point1 = VectorPoint::from_xy(1., 2.);
        let vector_point2 = VectorPoint::from_xy(4., 5.);
        let result = &vector_point1 / &vector_point2;
        assert_eq!(result.x, 0.25);
        assert_eq!(result.y, 0.4);
        assert_eq!(result.z, None);
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);

        // assign
        let mut vector_point1 = VectorPoint::from_xyz(1., 2., 3.);
        let vector_point2 = VectorPoint::from_xyz(4., 5., 6.);
        vector_point1 /= &vector_point2;
        assert_eq!(vector_point1.x, 0.25);
        assert_eq!(vector_point1.y, 0.4);
        assert_eq!(vector_point1.z, Some(0.5));
        assert_eq!(vector_point1.m, None);
        assert_eq!(vector_point1.t, None);
    }

    #[test]
    fn vector_point_div_f64() {
        let vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let float: f64 = 4.0;
        let result = &vector_point1 / float;
        assert_eq!(result.x, 0.25);
        assert_eq!(result.y, 0.5);
        assert_eq!(result.z, Some(0.75));
        assert_eq!(result.m, None);
        assert_eq!(result.t, None);

        // assign
        let mut vector_point1: VectorPoint =
            VectorPoint { x: 1.0, y: 2.0, z: Some(3.0), m: None, t: None };
        let float: f64 = 4.0;
        vector_point1 /= float;
        assert_eq!(vector_point1.x, 0.25);
        assert_eq!(vector_point1.y, 0.5);
        assert_eq!(vector_point1.z, Some(0.75));
        assert_eq!(vector_point1.m, None);
        assert_eq!(vector_point1.t, None);
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

    #[test]
    fn is_empty() {
        let vector_point1 = VectorPoint::from_xy(1.0, 2.0);
        assert!(!vector_point1.is_empty());
        let vector_point2 = VectorPoint::from_xyz(1.0, 2.0, 3.0);
        assert!(!vector_point2.is_empty());
        let vector_point3 = VectorPoint::from_xyz(0.0, 0.0, 0.0);
        assert!(vector_point3.is_empty());
        let vector_point4 = VectorPoint::from_xy(0.0, 0.0);
        assert!(vector_point4.is_empty());
    }

    #[test]
    fn get_face() {
        let point = VectorPoint::from_xyz(1.0, 2.0, 3.0);
        assert_eq!(point.face(0), 1.);
        assert_eq!(point.face(1), 2.);
        assert_eq!(point.face(2), 3.);
        let point = VectorPoint::from_xy(1.0, 2.0);
        assert_eq!(point.face(0), 1.);
        assert_eq!(point.face(1), 2.);
        assert_eq!(point.face(2), 0.);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn angle() {
        let point1 = VectorPoint::from_xyz(1.0, 0.0, 0.0);
        let point2 = VectorPoint::from_xyz(0.0, 1.0, 0.0);
        assert_eq!(point1.angle(&point2), 1.5707963267948966);

        let point1 = VectorPoint::from_xy(1.0, 0.0);
        let point2 = VectorPoint::from_xy(0.0, 1.0);
        assert_eq!(point1.angle(&point2), 1.5707963267948966);
    }

    #[test]
    fn abs() {
        let point = VectorPoint::from_xyz(1.0, 2.0, 3.0);
        assert_eq!(point.abs(), VectorPoint::from_xyz(1.0, 2.0, 3.0));

        let point = VectorPoint::from_xyz(-1.0, -2.0, -3.0);
        assert_eq!(point.abs(), VectorPoint::from_xyz(1.0, 2.0, 3.0));

        let point = VectorPoint::from_xy(1.0, 2.0);
        assert_eq!(point.abs(), VectorPoint::from_xy(1.0, 2.0));

        let point = VectorPoint::from_xy(-1.0, -2.0);
        assert_eq!(point.abs(), VectorPoint::from_xy(1.0, 2.0));
    }

    #[test]
    fn invert() {
        let point = VectorPoint::from_xyz(1.0, 2.0, 3.0);
        assert_eq!(point.invert(), VectorPoint::from_xyz(-1.0, -2.0, -3.0));

        let point = VectorPoint::from_xy(1.0, 2.0);
        assert_eq!(point.invert(), VectorPoint::from_xy(-1.0, -2.0));
    }

    #[test]
    fn normalize() {
        let mut point = VectorPoint::from_xyz(1.0, 2.0, 3.0);
        point.normalize();
        assert_eq!(
            point,
            VectorPoint::from_xyz(0.2672612419124244, 0.5345224838248488, 0.8017837257372732)
        );

        let mut point = VectorPoint::from_xyz(0.0, 0.0, 0.0);
        point.normalize();
        assert_eq!(point, VectorPoint::from_xyz(0.0, 0.0, 0.0));
    }

    #[test]
    fn largest_abs_component() {
        let point = VectorPoint::from_xyz(1.0, 2.0, 3.0);
        assert_eq!(point.largest_abs_component(), 2);
        let point = VectorPoint::from_xyz(3.0, 2.0, 1.0);
        assert_eq!(point.largest_abs_component(), 0);
        let point = VectorPoint::from_xyz(1.0, 3.0, 2.0);
        assert_eq!(point.largest_abs_component(), 1);
        let point = VectorPoint::from_xyz(2.0, 1.0, 3.0);
        assert_eq!(point.largest_abs_component(), 2);
    }

    #[test]
    fn intermediate() {
        let point1 = VectorPoint::from_xyz(1.0, 2.0, 3.0);
        let point2 = VectorPoint::from_xyz(4.0, 5.0, 6.0);
        assert_eq!(point1.intermediate(&point2, 0.5), VectorPoint::from_xyz(2.5, 3.5, 4.5));

        let point1 = VectorPoint::from_xy(1.0, 2.0);
        let point2 = VectorPoint::from_xy(4.0, 5.0);
        assert_eq!(point1.intermediate(&point2, 0.5), VectorPoint::from_xy(2.5, 3.5));
    }

    #[test]
    fn perpendicular() {
        let point = VectorPoint::from_xyz(1.0, 2.0, 3.0);
        assert_eq!(point.perpendicular(), VectorPoint::from_xyz(-2.0, 1.0, -2.0));

        let point = VectorPoint::from_xyz(3.0, 2.0, 1.0);
        assert_eq!(point.perpendicular(), VectorPoint::from_xyz(-2.0, 3.0, 0.0));

        let point = VectorPoint::from_xy(1.0, 2.0);
        assert_eq!(point.perpendicular(), VectorPoint::from_xy(-2., 1.));
    }
}
