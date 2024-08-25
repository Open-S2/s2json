/// Earth's radius in meters
pub const EARTH_RADIUS: f64 = 6_371_008.8;
/// Earth's equatorial radius in meters
pub const EARTH_RADIUS_EQUATORIAL: f64 = 6_378_137.0;
/// Earth's polar radius in meters
pub const EARTH_RADIUS_POLAR: f64 = 6_356_752.3;
/// The average circumference of the world in meters
pub const EARTH_CIRCUMFERENCE: f64 = 2.0 * core::f64::consts::PI * EARTH_RADIUS;

/// Mars' radius in meters
pub const MARS_RADIUS: f64 = 3_389_500.0;
/// Mars' equatorial radius in meters
pub const MARS_RADIUS_EQUATORIAL: f64 = 3_396_200.0;
/// Mars' polar radius in meters
pub const MARS_RADIUS_POLAR: f64 = 3_376_200.0;

/// 900913 (Web Mercator) constant
pub const A: f64 = 6_378_137.0;
/// 900913 (Web Mercator) max extent
pub const MAXEXTENT: f64 = 20_037_508.342789244;
/// 900913 (Web Mercator) maximum latitude
pub const MAXLAT: f64 = 85.0511287798;

/// convert radians to degrees
pub fn rad_to_deg(radians: f64) -> f64 {
    (radians * 180.0) / core::f64::consts::PI
}

/// convert degrees to radians
pub fn deg_to_rad(deg: f64) -> f64 {
    (deg * core::f64::consts::PI) / 180.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rad_to_deg() {
        assert_eq!(rad_to_deg(0.0), 0.0);
        assert_eq!(rad_to_deg(1.0), 57.29577951308232);
    }

    #[test]
    fn test_deg_to_rad() {
        assert_eq!(deg_to_rad(0.0), 0.0);
        assert_eq!(deg_to_rad(57.29577951308232), 1.0);
    }
}
