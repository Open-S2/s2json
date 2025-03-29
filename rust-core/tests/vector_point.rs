extern crate alloc;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;
    use s2json_core::*;

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

    #[test]
    fn test_get_traits() {
        #[derive(Debug, PartialEq, Clone)]
        struct Test {
            a: String,
        }
        let vp: VectorPoint<Test> =
            VectorPoint::new_xyz(1.0, 2.0, 3.0, Some(Test { a: "a".to_string() }));

        assert_eq!(vp.x(), 1.0);
        assert_eq!(vp.y(), 2.0);
        assert_eq!(vp.z(), 3.0);
        assert_eq!(vp.m(), Some(&Test { a: "a".to_string() }));
    }
}
