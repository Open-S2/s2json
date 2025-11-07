extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::vec;
    use core::cmp::Ordering;
    use s2json_core::*;
    use serde_json::json;

    #[test]
    fn test_vector_offset() {
        let offset = VectorOffsets::default();
        assert_eq!(offset, VectorOffsets::LineOffset(0.0));
        let offset: VectorOffsets = Default::default();
        assert_eq!(offset, VectorOffsets::LineOffset(0.0));
    }

    #[test]
    fn test_bounding_box_compares() {
        let bbox = BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 };
        assert_eq!(bbox, BBox { ..bbox });
        let bbox_3d = BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, far: 0.0, near: 0.0 };
        assert_eq!(bbox_3d, BBox3D { ..bbox_3d });

        let bbox_cont = BBOX::BBox(bbox);
        let bbox_3d_cont = BBOX::BBox3D(bbox_3d);

        assert_eq!(bbox_cont.cmp(&bbox_3d_cont), Ordering::Less);
        assert_eq!(bbox_cont.partial_cmp(&bbox_3d_cont), Some(Ordering::Less));
        assert_eq!(bbox_3d_cont.cmp(&bbox_cont), Ordering::Greater);
        assert_eq!(bbox_3d_cont.partial_cmp(&bbox_cont), Some(Ordering::Greater));
        assert_eq!(
            bbox_cont.cmp(&BBOX::BBox(BBox { left: 0.1, bottom: 0.1, right: 1.0, top: 1.0 })),
            Ordering::Less
        );
        assert_eq!(
            bbox_3d_cont.cmp(&BBOX::BBox3D(BBox3D {
                left: 0.1,
                bottom: 0.1,
                right: 1.0,
                top: 1.0,
                far: 0.0,
                near: 0.0,
            })),
            Ordering::Less
        );
    }

    #[test]
    fn test_bbox() {
        let bbox = BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 };
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 });
        let bbox_str = serde_json::to_string(&bbox).unwrap();
        assert_eq!(bbox_str, "[0.0,0.0,1.0,1.0]");
        let str_bbox: BBox = serde_json::from_str(&bbox_str).unwrap();
        assert_eq!(str_bbox, bbox);

        let default_bbox = BBox::default();
        assert_eq!(
            default_bbox,
            BBox { left: f64::MAX, bottom: f64::MAX, right: f64::MIN, top: f64::MIN }
        );

        let default_bbox_2 = BBOX::default();
        assert_eq!(
            default_bbox_2,
            BBOX::BBox(BBox { left: f64::MAX, bottom: f64::MAX, right: f64::MIN, top: f64::MIN })
        );
    }

    #[test]
    fn test_bbox_area() {
        let bbox = BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 };
        assert_eq!(bbox.area(), 1.0);
        let bbox = BBox { left: 0.0, bottom: 0.0, right: 10.0, top: 10.0 };
        assert_eq!(bbox.area(), 100.0);
        let bbox = BBox { left: -5.0, bottom: -5.0, right: 5.0, top: 5.0 };
        assert_eq!(bbox.area(), 100.0);
        let bbox = BBox { left: -10.0, bottom: -10.0, right: 0.0, top: 0.0 };
        assert_eq!(bbox.area(), 100.0);

        // no near-far use
        let bbox = BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, far: 0.0, near: 0.0 };
        assert_eq!(bbox.area(), 1.0);
        let bbox = BBox3D { left: 0.0, bottom: 0.0, right: 10.0, top: 10.0, far: 0.0, near: 0.0 };
        assert_eq!(bbox.area(), 100.0);
        let bbox = BBox3D { left: -5.0, bottom: -5.0, right: 5.0, top: 5.0, far: 0.0, near: 0.0 };
        assert_eq!(bbox.area(), 100.0);
        let bbox = BBox3D { left: -10.0, bottom: -10.0, right: 0.0, top: 0.0, far: 0.0, near: 0.0 };
        assert_eq!(bbox.area(), 100.0);
        // with near-far
        let bbox = BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, far: 1.0, near: 0.0 };
        assert_eq!(bbox.area(), 1.0);
        let bbox = BBox3D { left: 0.0, bottom: 0.0, right: 10.0, top: 10.0, far: 1.0, near: 0.0 };
        assert_eq!(bbox.area(), 100.0);
        let bbox = BBox3D { left: -5.0, bottom: -5.0, right: 5.0, top: 5.0, far: 1.0, near: 0.0 };
        assert_eq!(bbox.area(), 100.0);
        let bbox = BBox3D { left: -10.0, bottom: -10.0, right: 0.0, top: 0.0, far: 1.0, near: 0.0 };
        assert_eq!(bbox.area(), 100.0);
        // use near and far
        let bbox = BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, far: 1.0, near: -1.0 };
        assert_eq!(bbox.area(), 2.0);
    }

    #[test]
    fn test_bbox_inside() {
        let bbox = BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 };
        assert!(bbox.inside(&bbox));
        let bbox2 = BBox { left: 0.1, bottom: 0.1, right: 0.9, top: 0.9 };
        assert!(!bbox.inside(&bbox2));
        assert!(bbox2.inside(&bbox));
    }

    #[test]
    fn test_bbox3d_inside() {
        let bbox = BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, far: 0.0, near: 0.0 };
        assert!(bbox.inside(&bbox));
        let bbox2 = BBox3D { left: 0.1, bottom: 0.1, right: 0.9, top: 0.9, far: 0.0, near: 0.0 };
        assert!(!bbox.inside(&bbox2));
        assert!(bbox2.inside(&bbox));
    }

    #[test]
    fn test_bbox_mvalue() {
        let bbox = BBox { left: -2.2, bottom: -944.22, right: 1.0, top: 2.0 };

        let m_value: MValue = bbox.into();
        assert_eq!(
            m_value,
            MValue::from([
                ("left".into(), (-2.2_f64).into()),
                ("bottom".into(), (-944.22_f64).into()),
                ("right".into(), (1.0_f64).into()),
                ("top".into(), (2.0_f64).into()),
            ])
        );

        let back_to_bbox: BBox = m_value.into();
        assert_eq!(back_to_bbox, bbox);
    }

    #[test]
    fn test_bbox_mvalue_refs() {
        let bbox = BBox { left: -2.2, bottom: -944.22, right: 1.0, top: 2.0 };

        let m_value: MValue = (&bbox).into();
        assert_eq!(
            m_value,
            MValue::from([
                ("left".into(), (-2.2_f64).into()),
                ("bottom".into(), (-944.22_f64).into()),
                ("right".into(), (1.0_f64).into()),
                ("top".into(), (2.0_f64).into()),
            ])
        );

        let back_to_bbox: BBox = (&m_value).into();
        assert_eq!(back_to_bbox, bbox);
    }

    #[test]
    fn test_bbox_serialize() {
        let bbox = BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 };
        let bbox_str = serde_json::to_string(&bbox).unwrap();
        assert_eq!(bbox_str, "[0.0,0.0,1.0,1.0]");
    }

    #[test]
    fn test_bbox_deserialize() {
        let bbox_str = "[0.0,0.0,1.0,1.0]";
        let bbox: BBox = serde_json::from_str(bbox_str).unwrap();
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 });
    }

    #[test]
    fn test_bbox_clip() {
        let bbox = BBox::new(0., 0., 1., 1.);
        let bbox2 = BBox { left: 0.5, bottom: 0., right: 0.75, top: 1. };
        assert_eq!(bbox.clip(Axis::X, 0.5, 0.75), bbox2);
        let bbox2 = BBox { left: 0., bottom: 0.5, right: 1., top: 0.75 };
        assert_eq!(bbox.clip(Axis::Y, 0.5, 0.75), bbox2);
    }

    #[test]
    fn test_bbox_overlap() {
        let bbox = BBox::new(0., 0., 1., 1.);
        assert!(bbox.point_overlap(&VectorPoint::<MValue>::new(0.5, 0.5, None, None)));
        assert!(!bbox.point_overlap(&VectorPoint::<MValue>::new(2.0, 2.0, None, None)));
        let bbox2 = BBox { left: 0.5, bottom: 0.5, right: 1.5, top: 1.5 };
        assert_eq!(
            bbox.overlap(&bbox2),
            Some(BBox { left: 0.5, bottom: 0.5, right: 1.0, top: 1.0 })
        );
        let bbox3 = BBox { left: 2.0, bottom: 2.0, right: 3.0, top: 3.0 };
        assert_eq!(bbox3.overlap(&bbox), None);
    }

    #[test]
    fn test_bbox_merge() {
        let bbox = BBox::new(0., 0., 1., 1.);
        let bbox2 = BBox { left: 0.5, bottom: 0.5, right: 1.5, top: 1.5 };
        assert_eq!(bbox.merge(&bbox2), BBox { left: 0.0, bottom: 0.0, right: 1.5, top: 1.5 });
        assert_eq!(bbox2.merge(&bbox), BBox { left: 0.0, bottom: 0.0, right: 1.5, top: 1.5 });
        let bbox3 = BBox { left: 2.0, bottom: 2.0, right: 3.0, top: 3.0 };
        assert_eq!(bbox.merge(&bbox3), BBox { left: 0.0, bottom: 0.0, right: 3.0, top: 3.0 });
    }

    #[test]
    fn test_bbox_from_st_uv() {
        let bbox = BBox::from_st_zoom(0., 0., 0);
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1., top: 1. });

        let bbox = BBox::from_st_zoom(1., 0., 1);
        assert_eq!(bbox, BBox { left: 0.5, bottom: 0.0, right: 1., top: 0.5 });

        let bbox = BBox::from_st_zoom(2., 0., 2);
        assert_eq!(bbox, BBox { left: 0.5, bottom: 0.0, right: 0.75, top: 0.25 });

        let bbox = BBox::from_uv_zoom(0., 0., 0);
        assert_eq!(bbox, BBox { left: -1.0, bottom: -1.0, right: 1., top: 1. });

        let bbox = BBox::from_uv_zoom(1., 0., 1);
        assert_eq!(bbox, BBox { left: 0., bottom: -1.0, right: 1., top: 0. });

        let bbox = BBox::from_uv_zoom(2., 0., 2);
        assert_eq!(bbox, BBox { left: 0., bottom: -1.0, right: 0.5, top: -0.5 });
    }

    #[test]
    fn test_bbox_from_point() {
        let bbox = BBox::from_point(&VectorPoint::<MValue>::new(0., 0., None, None));
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 0.0, top: 0.0 });
    }

    #[test]
    fn test_bbox_from_linestring() {
        let bbox = BBox::from_linestring(&vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(1., 1.5, None, None),
        ]);
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.5 });

        let bbox = BBox::from_linestring(&vec![] as &[VectorPoint<MValue>]);
        assert_eq!(bbox, BBox::default());
    }

    #[test]
    fn test_bbox_from_multilinestring() {
        let bbox = BBox::from_multi_linestring(&vec![vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(1., 1.5, None, None),
        ]]);
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.5 });

        let bbox = BBox::from_multi_linestring(&vec![] as &[Vec<VectorPoint<MValue>>]);
        assert_eq!(bbox, BBox::default());
    }

    #[test]
    fn test_bbox_from_polygon() {
        let bbox = BBox::from_polygon(&vec![vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(2., 1.5, None, None),
        ]]);
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 2.0, top: 1.5 });

        let bbox = BBox::from_polygon(&vec![] as &[Vec<VectorPoint<MValue>>]);
        assert_eq!(bbox, BBox::default());
    }

    #[test]
    fn test_bbox_from_multipolygon() {
        let bbox = BBox::from_multi_polygon(&vec![
            vec![vec![
                VectorPoint::<MValue>::new(0., 0., None, None),
                VectorPoint::new(2., 1.5, None, None),
            ]],
            vec![vec![
                VectorPoint::new(0., 0., None, None),
                VectorPoint::new(-1., 3.5, None, None),
            ]],
        ]);
        assert_eq!(bbox, BBox { left: -1.0, bottom: 0.0, right: 2.0, top: 3.5 });

        let bbox = BBox::from_multi_polygon(&vec![] as &[Vec<Vec<VectorPoint<MValue>>>]);
        assert_eq!(bbox, BBox::default());
    }

    #[test]
    fn test_bbox3d() {
        let bbox = BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, near: 0.0, far: 1.0 };
        assert_eq!(
            bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, near: 0.0, far: 1.0 }
        );
        let bbox_str = serde_json::to_string(&bbox).unwrap();
        assert_eq!(bbox_str, "[0.0,0.0,1.0,1.0,0.0,1.0]");
        let str_bbox: BBox3D = serde_json::from_str(&bbox_str).unwrap();
        assert_eq!(str_bbox, bbox);

        let default_bbox = BBox3D::default();
        assert_eq!(
            default_bbox,
            BBox3D {
                left: f64::MAX,
                bottom: f64::MAX,
                right: f64::MIN,
                top: f64::MIN,
                near: f64::MAX,
                far: f64::MIN
            }
        );
    }

    #[test]
    fn test_bbox_3d_serialize() {
        let bbox = BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, near: 0.0, far: 1.0 };
        let bbox_str = serde_json::to_string(&bbox).unwrap();
        assert_eq!(bbox_str, "[0.0,0.0,1.0,1.0,0.0,1.0]");
    }

    #[test]
    fn test_bbox_3_d_deserialize() {
        let bbox_str = "[0.0,0.0,1.0,1.0,0.0,1.0]";
        let bbox: BBox3D = serde_json::from_str(bbox_str).unwrap();
        assert_eq!(
            bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, near: 0.0, far: 1.0 }
        );
    }

    #[test]
    fn test_bbox_3_d_overlap() {
        let bbox = BBox3D::new(0., 0., 1., 1., 0., 1.);
        assert!(bbox.point_overlap(&VectorPoint::<MValue>::new(0.5, 0.5, None, None)));
        assert!(!bbox.point_overlap(&VectorPoint::<MValue>::new(2.0, 2.0, None, None)));
        let bbox2 = BBox3D { left: 0.5, bottom: 0.5, right: 1.5, top: 1.5, near: 0.5, far: 1.5 };
        assert_eq!(
            bbox.overlap(&bbox2),
            Some(BBox3D { left: 0.5, bottom: 0.5, right: 1.0, top: 1.0, near: 0.5, far: 1.0 })
        );
        let bbox3 = BBox3D { left: 2.0, bottom: 2.0, right: 3.0, top: 3.0, near: 2.0, far: 3.0 };
        assert_eq!(bbox3.overlap(&bbox), None);
    }

    #[test]
    fn test_bbox_3_d_clip() {
        let bbox = BBox3D::new(0., 0., 1., 1., -1., 5.);
        let bbox2 = BBox3D { left: 0.5, bottom: 0., right: 0.75, top: 1., near: -1., far: 5. };
        assert_eq!(bbox.clip(Axis::X, 0.5, 0.75), bbox2);
        let bbox2 = BBox3D { left: 0., bottom: 0.5, right: 1., top: 0.75, near: -1., far: 5. };
        assert_eq!(bbox.clip(Axis::Y, 0.5, 0.75), bbox2);
    }

    #[test]
    fn test_bbox_3_d_merge() {
        let bbox = BBox3D::new(0., 0., 1., 1., 0., 1.);
        let bbox2 = BBox3D { left: 0.5, bottom: 0.5, right: 1.5, top: 1.5, near: 0.5, far: 1.5 };
        assert_eq!(
            bbox.merge(&bbox2),
            BBox3D { left: 0.0, bottom: 0.0, right: 1.5, top: 1.5, near: 0.0, far: 1.5 }
        );
        assert_eq!(
            bbox2.merge(&bbox),
            BBox3D { left: 0.0, bottom: 0.0, right: 1.5, top: 1.5, near: 0.0, far: 1.5 }
        );
        let bbox3 = BBox3D { left: 2.0, bottom: 2.0, right: 3.0, top: 3.0, near: 2.0, far: 3.0 };
        assert_eq!(
            bbox.merge(&bbox3),
            BBox3D { left: 0.0, bottom: 0.0, right: 3.0, top: 3.0, near: 0.0, far: 3.0 }
        );
    }

    #[test]
    fn test_bbox_3_d_from_point() {
        let bbox = BBox3D::from_point(&VectorPoint::<MValue>::new(0., 0., None, None));
        assert_eq!(
            bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 0.0, top: 0.0, near: f64::MAX, far: f64::MIN }
        );
    }

    #[test]
    fn test_bbox_3_d_from_linestring() {
        let bbox = BBox3D::from_linestring(&vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(1., 1.5, None, None),
        ]);
        assert_eq!(
            bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.5, near: f64::MAX, far: f64::MIN }
        );

        let bbox = BBox3D::from_linestring(&vec![] as &[VectorPoint<MValue>]);
        assert_eq!(bbox, BBox3D::default());
    }

    #[test]
    fn test_bbox_3_d_from_multilinestring() {
        let bbox = BBox3D::from_multi_linestring(&vec![vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(1., 1.5, None, None),
        ]]);
        assert_eq!(
            bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.5, near: f64::MAX, far: f64::MIN }
        );

        let bbox = BBox3D::from_multi_linestring(&vec![] as &[Vec<VectorPoint<MValue>>]);
        assert_eq!(bbox, BBox3D::default());
    }

    #[test]
    fn test_bbox_3_d_from_polygon() {
        let bbox = BBox3D::from_polygon(&vec![vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(2., 1.5, None, None),
        ]]);
        assert_eq!(
            bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 2.0, top: 1.5, near: f64::MAX, far: f64::MIN }
        );

        let bbox = BBox3D::from_polygon(&vec![] as &[Vec<VectorPoint<MValue>>]);
        assert_eq!(bbox, BBox3D::default());
    }

    #[test]
    fn test_bbox_3_d_from_multipolygon() {
        let bbox = BBox3D::from_multi_polygon(&vec![
            vec![vec![
                VectorPoint::<MValue>::new(0., 0., None, None),
                VectorPoint::new(2., 1.5, None, None),
            ]],
            vec![vec![
                VectorPoint::new(0., 0., None, None),
                VectorPoint::new(-1., 3.5, None, None),
            ]],
        ]);
        assert_eq!(
            bbox,
            BBox3D { left: -1.0, bottom: 0.0, right: 2.0, top: 3.5, near: f64::MAX, far: f64::MIN }
        );

        let bbox = BBox3D::from_multi_polygon(&vec![] as &[Vec<Vec<VectorPoint<MValue>>>]);
        assert_eq!(bbox, BBox3D::default());
    }

    #[test]
    fn test_bbox_3_d_extend_from_point() {
        let mut bbox = BBox3D::default();
        bbox.extend_from_point(&VectorPoint::<MValue>::new(20., -4., None, None));
        assert_eq!(
            bbox,
            BBox3D {
                left: 20.0,
                bottom: -4.0,
                right: 20.0,
                top: -4.0,
                near: f64::MAX,
                far: f64::MIN
            }
        );
    }

    #[test]
    fn test_bbox_3_d_from_st_uv() {
        let bbox = BBox3D::from_st_zoom(0., 0., 0);
        assert_eq!(
            bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 1., top: 1., near: f64::MAX, far: f64::MIN }
        );

        let bbox = BBox3D::from_st_zoom(1., 0., 1);
        assert_eq!(
            bbox,
            BBox3D { left: 0.5, bottom: 0.0, right: 1., top: 0.5, near: f64::MAX, far: f64::MIN }
        );

        let bbox = BBox3D::from_st_zoom(2., 0., 2);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.5,
                bottom: 0.0,
                right: 0.75,
                top: 0.25,
                near: f64::MAX,
                far: f64::MIN
            }
        );

        let bbox = BBox3D::from_uv_zoom(0., 0., 0);
        assert_eq!(
            bbox,
            BBox3D { left: -1.0, bottom: -1.0, right: 1., top: 1., near: f64::MAX, far: f64::MIN }
        );

        let bbox = BBox3D::from_uv_zoom(1., 0., 1);
        assert_eq!(
            bbox,
            BBox3D { left: 0., bottom: -1.0, right: 1., top: 0., near: f64::MAX, far: f64::MIN }
        );

        let bbox = BBox3D::from_uv_zoom(2., 0., 2);
        assert_eq!(
            bbox,
            BBox3D { left: 0., bottom: -1.0, right: 0.5, top: -0.5, near: f64::MAX, far: f64::MIN }
        );
    }

    #[test]
    fn test_bbox_3_d_from_bbox() {
        let bbox: BBox3D = BBox::new(0., 0., 1., 1.).into();
        assert_eq!(
            bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, near: 0.0, far: 0.0 }
        );
    }

    #[test]
    fn test_bbox_3_d_to_bbox() {
        let bbox_3d = BBox3D::new(0., 0., 1., 1., 0., 1.);
        let bbox: BBox = bbox_3d.into();
        assert_eq!(bbox, BBox::new(0., 0., 1., 1.));
    }

    #[test]
    fn test_point_geometry() {
        let point = PointGeometry {
            _type: "Point".into(),
            coordinates: Point(0.0, 0.0),
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            point,
            PointGeometry {
                _type: "Point".into(),
                coordinates: Point(0.0, 0.0),
                m_values: None,
                bbox: None
            }
        );
        let point_str = serde_json::to_string(&point).unwrap();
        assert_eq!(point_str, "{\"type\":\"Point\",\"coordinates\":[0.0,0.0]}");
        let str_point: PointGeometry = serde_json::from_str(&point_str).unwrap();
        assert_eq!(str_point, point);
    }

    #[test]
    fn test_geometry_default() {
        let geo = Geometry::default();
        assert_eq!(geo, Geometry::Point(PointGeometry::default()));

        let default_instance: Geometry = Default::default();
        assert_eq!(geo, default_instance);
    }

    #[test]
    fn test_point3d_geometry() {
        let point = Point3DGeometry {
            _type: "Point3D".into(),
            coordinates: Point3D(0.0, 0.0, 0.0),
            m_values: None,
            bbox: Some(BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 1.0,
                top: 1.0,
                near: 0.0,
                far: 1.0,
            }),
        };
        assert_eq!(
            point,
            Point3DGeometry {
                _type: "Point3D".into(),
                coordinates: Point3D(0.0, 0.0, 0.0),
                m_values: None,
                bbox: Some(BBox3D {
                    left: 0.0,
                    bottom: 0.0,
                    right: 1.0,
                    top: 1.0,
                    near: 0.0,
                    far: 1.0
                })
            }
        );
        let point_str = serde_json::to_string(&point).unwrap();
        assert_eq!(
            point_str,
            "{\"type\":\"Point3D\",\"coordinates\":[0.0,0.0,0.0],\"bbox\":[0.0,0.0,1.0,1.0,0.0,1.\
             0]}"
        );
        let str_point: Point3DGeometry = serde_json::from_str(&point_str).unwrap();
        assert_eq!(str_point, point);
    }

    #[test]
    fn test_line_string_geometry() {
        let line = LineStringGeometry {
            _type: "LineString".into(),
            coordinates: vec![Point(0.0, 0.0), Point(1.0, 1.0)],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            line,
            LineStringGeometry {
                _type: "LineString".into(),
                coordinates: vec![Point(0.0, 0.0), Point(1.0, 1.0)],
                m_values: None,
                bbox: None
            }
        );
        let line_str = serde_json::to_string(&line).unwrap();
        assert_eq!(line_str, "{\"type\":\"LineString\",\"coordinates\":[[0.0,0.0],[1.0,1.0]]}");
        let str_line: LineStringGeometry = serde_json::from_str(&line_str).unwrap();
        assert_eq!(str_line, line);
    }

    #[test]
    fn test_line_string3d_geometry() {
        let line = LineString3DGeometry::<MValue> {
            _type: "LineString3D".into(),
            coordinates: vec![Point3D(0.0, 0.0, 0.0), Point3D(1.0, 1.0, 1.0)],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            line,
            LineString3DGeometry {
                _type: "LineString3D".into(),
                coordinates: vec![Point3D(0.0, 0.0, 0.0), Point3D(1.0, 1.0, 1.0)],
                m_values: None,
                bbox: None
            }
        );
        let line_str = serde_json::to_string(&line).unwrap();
        assert_eq!(
            line_str,
            "{\"type\":\"LineString3D\",\"coordinates\":[[0.0,0.0,0.0],[1.0,1.0,1.0]]}"
        );
        let str_line: LineString3DGeometry = serde_json::from_str(&line_str).unwrap();
        assert_eq!(str_line, line);
    }

    #[test]
    fn test_multi_point_geometry() {
        let multi_point = MultiPointGeometry {
            _type: "MultiPoint".into(),
            coordinates: vec![Point(0.0, 0.0), Point(1.0, 1.0)],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            multi_point,
            MultiPointGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![Point(0.0, 0.0), Point(1.0, 1.0)],
                m_values: None,
                bbox: None
            }
        );
        let multi_point_str = serde_json::to_string(&multi_point).unwrap();
        assert_eq!(
            multi_point_str,
            "{\"type\":\"MultiPoint\",\"coordinates\":[[0.0,0.0],[1.0,1.0]]}"
        );
        let str_multi_point: MultiPointGeometry = serde_json::from_str(&multi_point_str).unwrap();
        assert_eq!(str_multi_point, multi_point);
    }

    #[test]
    fn test_multi_point3d_geometry() {
        let multi_point = MultiPoint3DGeometry {
            _type: "MultiPoint3D".into(),
            coordinates: vec![Point3D(0.0, 0.0, 0.0), Point3D(1.0, 1.0, 1.0)],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            multi_point,
            MultiPoint3DGeometry {
                _type: "MultiPoint3D".into(),
                coordinates: vec![Point3D(0.0, 0.0, 0.0), Point3D(1.0, 1.0, 1.0)],
                m_values: None,
                bbox: None
            }
        );
        let multi_point_str = serde_json::to_string(&multi_point).unwrap();
        assert_eq!(
            multi_point_str,
            "{\"type\":\"MultiPoint3D\",\"coordinates\":[[0.0,0.0,0.0],[1.0,1.0,1.0]]}"
        );
        let str_multi_point: MultiPoint3DGeometry = serde_json::from_str(&multi_point_str).unwrap();
        assert_eq!(str_multi_point, multi_point);
    }

    #[test]
    fn test_polygon_geometry() {
        let polygon = PolygonGeometry {
            _type: "Polygon".into(),
            coordinates: vec![vec![Point(0.0, 0.0), Point(1.0, 1.0), Point(0.0, 1.0)]],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            polygon,
            PolygonGeometry {
                _type: "Polygon".into(),
                coordinates: vec![vec![Point(0.0, 0.0), Point(1.0, 1.0), Point(0.0, 1.0)]],
                m_values: None,
                bbox: None
            }
        );
        let polygon_str = serde_json::to_string(&polygon).unwrap();
        assert_eq!(
            polygon_str,
            "{\"type\":\"Polygon\",\"coordinates\":[[[0.0,0.0],[1.0,1.0],[0.0,1.0]]]}"
        );
        let str_polygon: PolygonGeometry = serde_json::from_str(&polygon_str).unwrap();
        assert_eq!(str_polygon, polygon);
    }

    #[test]
    fn test_polygon3d_geometry() {
        let polygon = Polygon3DGeometry {
            _type: "Polygon3D".into(),
            coordinates: vec![vec![
                Point3D(0.0, 0.0, 0.0),
                Point3D(1.0, 1.0, 1.0),
                Point3D(0.0, 1.0, 1.0),
            ]],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            polygon,
            Polygon3DGeometry {
                _type: "Polygon3D".into(),
                coordinates: vec![vec![
                    Point3D(0.0, 0.0, 0.0),
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(0.0, 1.0, 1.0)
                ]],
                m_values: None,
                bbox: None
            }
        );
        let polygon_str = serde_json::to_string(&polygon).unwrap();
        assert_eq!(
            polygon_str,
            "{\"type\":\"Polygon3D\",\"coordinates\":[[[0.0,0.0,0.0],[1.0,1.0,1.0],[0.0,1.0,1.\
             0]]]}"
        );
        let str_polygon: Polygon3DGeometry = serde_json::from_str(&polygon_str).unwrap();
        assert_eq!(str_polygon, polygon);
    }

    #[test]
    fn test_multi_polygon_geometry() {
        let multi_polygon = MultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            coordinates: vec![vec![vec![Point(0.0, 0.0), Point(1.0, 1.0), Point(0.0, 1.0)]]],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            multi_polygon,
            MultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                coordinates: vec![vec![vec![Point(0.0, 0.0), Point(1.0, 1.0), Point(0.0, 1.0)]]],
                m_values: None,
                bbox: None
            }
        );
        let multi_polygon_str = serde_json::to_string(&multi_polygon).unwrap();
        assert_eq!(
            multi_polygon_str,
            "{\"type\":\"MultiPolygon\",\"coordinates\":[[[[0.0,0.0],[1.0,1.0],[0.0,1.0]]]]}"
        );
        let str_multi_polygon: MultiPolygonGeometry =
            serde_json::from_str(&multi_polygon_str).unwrap();
        assert_eq!(str_multi_polygon, multi_polygon);
    }

    #[test]
    fn test_multi_polygon3d_geometry() {
        let multi_polygon = MultiPolygon3DGeometry {
            _type: "MultiPolygon3D".into(),
            coordinates: vec![vec![vec![
                Point3D(0.0, 0.0, 0.0),
                Point3D(1.0, 1.0, 1.0),
                Point3D(0.0, 1.0, 1.0),
            ]]],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            multi_polygon,
            MultiPolygon3DGeometry {
                _type: "MultiPolygon3D".into(),
                coordinates: vec![vec![vec![
                    Point3D(0.0, 0.0, 0.0),
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(0.0, 1.0, 1.0)
                ]]],
                m_values: None,
                bbox: None
            }
        );
        let multi_polygon_str = serde_json::to_string(&multi_polygon).unwrap();
        assert_eq!(
            multi_polygon_str,
            "{\"type\":\"MultiPolygon3D\",\"coordinates\":[[[[0.0,0.0,0.0],[1.0,1.0,1.0],[0.0,1.0,\
             1.0]]]]}"
        );
        let str_multi_polygon: MultiPolygon3DGeometry =
            serde_json::from_str(&multi_polygon_str).unwrap();
        assert_eq!(str_multi_polygon, multi_polygon);
    }

    #[test]
    fn test_vector_geometry_default() {
        let default = VectorGeometry::default();
        assert_eq!(default, VectorGeometry::Point(VectorPointGeometry::default()));

        let default_instance: VectorGeometry = Default::default();
        assert_eq!(default, default_instance);
    }

    #[test]
    fn test_vector_geometry_bbox() {
        let vgt_point: VectorGeometry = VectorGeometry::Point(VectorPointGeometry {
            _type: "Point".into(),
            coordinates: VectorPoint { x: 0.0, y: 1.0, z: Some(2.0), m: None, t: None },
            bbox: Some(BBox3D {
                left: -1.0,
                bottom: -2.0,
                right: -3.0,
                top: -4.0,
                near: -5.0,
                far: -6.0,
            }),
            is_3d: true,
            offset: None,
            vec_bbox: Some(BBox3D {
                left: 0.0,
                bottom: 1.0,
                right: 0.0,
                top: 1.0,
                near: 2.0,
                far: 2.0,
            }),
            indices: None,
            tessellation: None,
        });
        assert_eq!(vgt_point.bbox().unwrap(), BBox3D::new(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0));
        assert_eq!(vgt_point.vec_bbox().unwrap(), BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 2.0));
        let vgt_multi_point: VectorGeometry =
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![VectorPoint { x: 0.0, y: 1.0, z: Some(2.0), m: None, t: None }],
                bbox: Some(BBox3D {
                    left: -1.0,
                    bottom: -2.0,
                    right: -3.0,
                    top: -4.0,
                    near: -5.0,
                    far: -6.0,
                }),
                is_3d: true,
                offset: None,
                vec_bbox: Some(BBox3D {
                    left: 0.0,
                    bottom: 1.0,
                    right: 0.0,
                    top: 1.0,
                    near: 2.0,
                    far: 2.0,
                }),
                indices: None,
                tessellation: None,
            });
        assert_eq!(
            vgt_multi_point.bbox().unwrap(),
            BBox3D::new(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0)
        );
        assert_eq!(vgt_multi_point.vec_bbox().unwrap(), BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 2.0));
        let vgt_line_string: VectorGeometry =
            VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "LineString".into(),
                coordinates: vec![VectorPoint { x: 0.0, y: 1.0, z: Some(2.0), m: None, t: None }],
                bbox: Some(BBox3D {
                    left: -1.0,
                    bottom: -2.0,
                    right: -3.0,
                    top: -4.0,
                    near: -5.0,
                    far: -6.0,
                }),
                is_3d: true,
                offset: None,
                vec_bbox: Some(BBox3D {
                    left: 0.0,
                    bottom: 1.0,
                    right: 0.0,
                    top: 1.0,
                    near: 2.0,
                    far: 2.0,
                }),
                indices: None,
                tessellation: None,
            });
        assert_eq!(
            vgt_line_string.bbox().unwrap(),
            BBox3D::new(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0)
        );
        assert_eq!(vgt_line_string.vec_bbox().unwrap(), BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 2.0));
        let vgt_multi_line_string: VectorGeometry =
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiLineString".into(),
                coordinates: vec![vec![VectorPoint {
                    x: 0.0,
                    y: 1.0,
                    z: Some(2.0),
                    m: None,
                    t: None,
                }]],
                bbox: Some(BBox3D {
                    left: -1.0,
                    bottom: -2.0,
                    right: -3.0,
                    top: -4.0,
                    near: -5.0,
                    far: -6.0,
                }),
                is_3d: true,
                offset: None,
                vec_bbox: Some(BBox3D {
                    left: 0.0,
                    bottom: 1.0,
                    right: 0.0,
                    top: 1.0,
                    near: 2.0,
                    far: 2.0,
                }),
                indices: None,
                tessellation: None,
            });
        assert_eq!(
            vgt_multi_line_string.bbox().unwrap(),
            BBox3D::new(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0)
        );
        assert_eq!(
            vgt_multi_line_string.vec_bbox().unwrap(),
            BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 2.0)
        );
        let vgt_polygon: VectorGeometry = VectorGeometry::Polygon(VectorPolygonGeometry {
            _type: "Polygon".into(),
            coordinates: vec![vec![VectorPoint { x: 0.0, y: 1.0, z: Some(2.0), m: None, t: None }]],
            bbox: Some(BBox3D {
                left: -1.0,
                bottom: -2.0,
                right: -3.0,
                top: -4.0,
                near: -5.0,
                far: -6.0,
            }),
            is_3d: true,
            offset: None,
            vec_bbox: Some(BBox3D {
                left: 0.0,
                bottom: 1.0,
                right: 0.0,
                top: 1.0,
                near: 2.0,
                far: 2.0,
            }),
            indices: None,
            tessellation: None,
        });
        assert_eq!(vgt_polygon.bbox().unwrap(), BBox3D::new(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0));
        assert_eq!(vgt_polygon.vec_bbox().unwrap(), BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 2.0));
        let vgt_multi_polygon: VectorGeometry =
            VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                coordinates: vec![vec![vec![VectorPoint {
                    x: 0.0,
                    y: 1.0,
                    z: Some(2.0),
                    m: None,
                    t: None,
                }]]],
                bbox: Some(BBox3D {
                    left: -1.0,
                    bottom: -2.0,
                    right: -3.0,
                    top: -4.0,
                    near: -5.0,
                    far: -6.0,
                }),
                is_3d: true,
                offset: None,
                vec_bbox: Some(BBox3D {
                    left: 0.0,
                    bottom: 1.0,
                    right: 0.0,
                    top: 1.0,
                    near: 2.0,
                    far: 2.0,
                }),
                indices: None,
                tessellation: None,
            });
        assert_eq!(
            vgt_multi_polygon.bbox().unwrap(),
            BBox3D::new(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0)
        );
        assert_eq!(
            vgt_multi_polygon.vec_bbox().unwrap(),
            BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 2.0)
        );
    }

    #[test]
    fn from_string() {
        let geo_str = json!({
            "type": "Point",
            "coordinates": [0, 0],
        })
        .to_string();
        let _feature: Geometry = serde_json::from_str(&geo_str).unwrap();

        let feature_str = json!({
            "type": "Feature",
            "properties": { "a": 1 },
            "geometry": {
                "type": "Point",
                "coordinates": [0, 0],
            }
        })
        .to_string();
        let _feature: Feature = serde_json::from_str(&feature_str).unwrap();

        let json_string = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "properties": { "a": 1 },
                    "geometry": {
                        "type": "Point",
                        "coordinates": [0, 0]
                    }
                },
                {
                    "type": "Feature",
                    "properties": { "b": 2 },
                    "geometry": {
                        "type": "Point3D",
                        "coordinates": [45, 45, 1]
                    }
                },
                {
                    "type": "Feature",
                    "properties": { "c": 3 },
                    "geometry": {
                        "type": "MultiPoint",
                        "coordinates": [
                            [-45, -45],
                            [-45, 45]
                        ]
                    }
                },
                {
                    "type": "Feature",
                    "properties": { "d": 4 },
                    "geometry": {
                        "type": "MultiPoint3D",
                        "coordinates": [
                            [45, -45, 1],
                            [-180, 20, 2]
                        ]
                    }
                }
            ]
        }"#;
        let data: FeatureCollection = serde_json::from_str(json_string).unwrap();
        assert_eq!(data.features.len(), 4);

        let data2: JSONCollection = serde_json::from_str(json_string).unwrap();
        if let JSONCollection::FeatureCollection(fc) = data2 {
            assert_eq!(fc.features.len(), 4);
            if let Features::Feature(first_feature) = &fc.features[0] {
                assert_eq!(first_feature.id, None);
                assert_eq!(first_feature._type, "Feature".into());
                assert_eq!(
                    first_feature.geometry,
                    Geometry::Point(PointGeometry {
                        _type: "Point".into(),
                        coordinates: Point(0.0, 0.0),
                        ..Default::default()
                    })
                );
            } else {
                panic!("Expected Feature");
            }
        } else {
            panic!("Expected FeatureCollection");
        }
    }

    #[test]
    fn vector_geometry_new_point() {
        let point = VectorPoint::from_xy(0.5, 0.75);
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let geometry = VectorGeometry::new_point(point.clone(), Some(bbox));

        assert_eq!(
            geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: point.clone(),
                bbox: Some(bbox),
                is_3d: false,
                ..Default::default()
            })
        );

        assert_eq!(geometry.point(), Some(&point));
    }

    #[test]
    fn vector_geometry_new_point_3d() {
        let point = VectorPoint::from_xyz(0.5, 0.75, 1.0);
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let geometry = VectorGeometry::new_point(point.clone(), Some(bbox));

        assert_eq!(
            geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: point.clone(),
                bbox: Some(bbox),
                is_3d: true,
                ..Default::default()
            })
        );

        assert_eq!(geometry.point(), Some(&point));
        assert_eq!(geometry.multipoint(), None);
        assert_eq!(geometry.linestring(), None);
        assert_eq!(geometry.multilinestring(), None);
        assert_eq!(geometry.polygon(), None);
        assert_eq!(geometry.multipolygon(), None);
    }

    #[test]
    fn vector_geometry_new_multipoint() {
        let multipoint = vec![VectorPoint::from_xy(0.5, 0.75), VectorPoint::from_xy(1.75, 2.5)];
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let mut geometry = VectorGeometry::new_multipoint(multipoint.clone(), Some(bbox));
        // doesn't add them
        geometry.set_tess(vec![0.0, 1.0, 2.0, 3.0]);
        geometry.set_indices(vec![0, 1, 2, 3]);

        assert_eq!(
            geometry,
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                coordinates: multipoint.clone(),
                bbox: Some(bbox),
                is_3d: false,
                ..Default::default()
            })
        );

        assert_eq!(geometry.point(), None);
        assert_eq!(geometry.multipoint(), Some(&multipoint));
    }

    #[test]
    fn vector_geometry_new_multipoint_3d() {
        let multipoint =
            vec![VectorPoint::from_xyz(0.5, 0.75, -1.0), VectorPoint::from_xyz(1.75, 2.5, 1.)];
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let geometry = VectorGeometry::new_multipoint(multipoint.clone(), Some(bbox));

        assert_eq!(
            geometry,
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                coordinates: multipoint.clone(),
                bbox: Some(bbox),
                is_3d: true,
                ..Default::default()
            })
        );

        assert_eq!(geometry.multipoint(), Some(&multipoint));
    }

    #[test]
    fn vector_geometry_new_line_string() {
        let line_string = vec![VectorPoint::from_xy(0.5, 0.75), VectorPoint::from_xy(1.75, 2.5)];
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let geometry = VectorGeometry::new_linestring(line_string.clone(), Some(bbox));

        assert_eq!(
            geometry,
            VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "LineString".into(),
                coordinates: line_string.clone(),
                bbox: Some(bbox),
                is_3d: false,
                ..Default::default()
            })
        );

        assert_eq!(geometry.linestring(), Some(&line_string));
    }

    #[test]
    fn vector_geometry_new_linestring_3d() {
        let linestring =
            vec![VectorPoint::from_xyz(0.5, 0.75, -1.0), VectorPoint::from_xyz(1.75, 2.5, 1.)];
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let geometry = VectorGeometry::new_linestring(linestring.clone(), Some(bbox));

        assert_eq!(
            geometry,
            VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "LineString".into(),
                coordinates: linestring.clone(),
                bbox: Some(bbox),
                is_3d: true,
                ..Default::default()
            })
        );

        assert_eq!(geometry.linestring(), Some(&linestring));
    }

    #[test]
    fn vector_geometry_new_multi_line_string() {
        let multi_line_string = vec![
            vec![VectorPoint::from_xy(0.5, 0.75), VectorPoint::from_xy(1.75, 2.5)],
            vec![VectorPoint::from_xy(1.5, 2.75), VectorPoint::from_xy(3.75, 4.5)],
        ];
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let geometry = VectorGeometry::new_multilinestring(multi_line_string.clone(), Some(bbox));

        assert_eq!(
            geometry,
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiLineString".into(),
                coordinates: multi_line_string.clone(),
                bbox: Some(bbox),
                is_3d: false,
                ..Default::default()
            })
        );

        assert_eq!(geometry.multilinestring(), Some(&multi_line_string));
    }

    #[test]
    fn vector_geometry_new_multi_line_string_3d() {
        let multi_line_string = vec![
            vec![VectorPoint::from_xyz(0.5, 0.75, -1.), VectorPoint::from_xyz(1.75, 2.5, -2.)],
            vec![VectorPoint::from_xyz(1.5, 2.75, 1.), VectorPoint::from_xyz(3.75, 4.5, 2.)],
        ];
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let geometry = VectorGeometry::new_multilinestring(multi_line_string.clone(), Some(bbox));

        assert_eq!(
            geometry,
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiLineString".into(),
                coordinates: multi_line_string.clone(),
                bbox: Some(bbox),
                is_3d: true,
                ..Default::default()
            })
        );

        assert_eq!(geometry.multilinestring(), Some(&multi_line_string));
    }

    #[test]
    fn vector_geometry_new_polygon() {
        let polygon = vec![
            vec![VectorPoint::from_xy(0.5, 0.75), VectorPoint::from_xy(1.75, 2.5)],
            vec![VectorPoint::from_xy(1.5, 2.75), VectorPoint::from_xy(3.75, 4.5)],
        ];
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let mut geometry = VectorGeometry::new_polygon(polygon.clone(), Some(bbox));
        geometry.set_tess(vec![0.0, 1.0, 2.0, 3.0]);
        geometry.set_indices(vec![0, 1, 2, 3]);

        assert_eq!(
            geometry,
            VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: "Polygon".into(),
                coordinates: polygon.clone(),
                bbox: Some(bbox),
                is_3d: false,
                tessellation: Some(vec![0.0, 1.0, 2.0, 3.0]),
                indices: Some(vec![0, 1, 2, 3]),
                ..Default::default()
            })
        );

        assert_eq!(geometry.polygon(), Some(&polygon));
    }

    #[test]
    fn vector_geometry_new_polygon_3d() {
        let polygon_3d = vec![
            vec![VectorPoint::from_xyz(0.5, 0.75, -1.), VectorPoint::from_xyz(1.75, 2.5, -2.)],
            vec![VectorPoint::from_xyz(1.5, 2.75, 1.), VectorPoint::from_xyz(3.75, 4.5, 2.)],
        ];
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let geometry = VectorGeometry::new_polygon(polygon_3d.clone(), Some(bbox));

        assert_eq!(
            geometry,
            VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: "Polygon".into(),
                coordinates: polygon_3d.clone(),
                bbox: Some(bbox),
                is_3d: true,
                ..Default::default()
            })
        );

        assert_eq!(geometry.polygon(), Some(&polygon_3d));
    }

    #[test]
    fn vector_geometry_new_multipolygon() {
        let multipolygon = vec![
            vec![
                vec![VectorPoint::from_xy(0.5, 0.75), VectorPoint::from_xy(1.75, 2.5)],
                vec![VectorPoint::from_xy(1.5, 2.75), VectorPoint::from_xy(3.75, 4.5)],
            ],
            vec![
                vec![VectorPoint::from_xy(1.5, 2.75), VectorPoint::from_xy(3.75, 4.5)],
                vec![VectorPoint::from_xy(5.5, 6.75), VectorPoint::from_xy(7.75, 8.5)],
            ],
        ];
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let mut geometry = VectorGeometry::new_multipolygon(multipolygon.clone(), Some(bbox));
        geometry.set_tess(vec![0.0, 1.0, 2.0, 3.0]);
        geometry.set_indices(vec![0, 1, 2, 3]);

        assert_eq!(
            geometry,
            VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                coordinates: multipolygon.clone(),
                bbox: Some(bbox),
                is_3d: false,
                tessellation: Some(vec![0.0, 1.0, 2.0, 3.0]),
                indices: Some(vec![0, 1, 2, 3]),
                ..Default::default()
            })
        );

        assert_eq!(geometry.multipolygon(), Some(&multipolygon));
    }

    #[test]
    fn vector_geometry_new_multipolygon_3d() {
        let multipolygon = vec![
            vec![
                vec![VectorPoint::from_xyz(0.5, 0.75, -1.), VectorPoint::from_xyz(1.75, 2.5, -2.)],
                vec![VectorPoint::from_xyz(1.5, 2.75, -3.), VectorPoint::from_xyz(3.75, 4.5, -4.)],
            ],
            vec![
                vec![VectorPoint::from_xyz(1.5, 2.75, 1.), VectorPoint::from_xyz(3.75, 4.5, 2.)],
                vec![VectorPoint::from_xyz(5.5, 6.75, 3.), VectorPoint::from_xyz(7.75, 8.5, 4.)],
            ],
        ];
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let geometry = VectorGeometry::new_multipolygon(multipolygon.clone(), Some(bbox));

        assert_eq!(
            geometry,
            VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                coordinates: multipolygon.clone(),
                bbox: Some(bbox),
                is_3d: true,
                ..Default::default()
            })
        );

        assert_eq!(geometry.multipolygon(), Some(&multipolygon));
    }

    #[test]
    fn point_or_point3d() {
        let point = Point(0., 1.);
        let point_or_point3d: PointOrPoint3D = point.into();
        assert_eq!(point_or_point3d, PointOrPoint3D(0., 1., None));

        let point = Point3D(0., 1., 2.);
        let point_or_point3d: PointOrPoint3D = point.into();
        assert_eq!(point_or_point3d, PointOrPoint3D(0., 1., Some(2.)));
    }

    #[test]
    fn point_trait_gets() {
        let point = Point(0., 1.);
        assert_eq!(point.x(), 0.);
        assert_eq!(point.y(), 1.);
        assert_eq!(point.xy(), (0., 1.));
        assert_eq!(point.z(), None);

        let point_3d = Point3D(0., 1., 2.);
        assert_eq!(point_3d.x(), 0.);
        assert_eq!(point_3d.y(), 1.);
        assert_eq!(point_3d.z(), Some(2.));

        let point_or_point3d: PointOrPoint3D = point.into();
        assert_eq!(point_or_point3d.x(), 0.);
        assert_eq!(point_or_point3d.y(), 1.);
        assert_eq!(point_or_point3d.z(), None);

        let point_or_point3d: PointOrPoint3D = point_3d.into();
        assert_eq!(point_or_point3d.x(), 0.);
        assert_eq!(point_or_point3d.y(), 1.);
        assert_eq!(point_or_point3d.z(), Some(2.));
    }

    #[test]
    fn point_trait_sets() {
        let mut point = Point(0., 1.);
        point.set_x(10.);
        point.set_y(20.);
        assert_eq!(point.x(), 10.);
        assert_eq!(point.y(), 20.);
        assert_eq!(point.z(), None);
        point.set_xy(30., 40.);
        assert_eq!(point.x(), 30.);
        assert_eq!(point.y(), 40.);
        assert_eq!(point.z(), None);

        let point: Point = (&Point3D(0., 1., 2.)).into();
        assert_eq!(point.x(), 0.);
        assert_eq!(point.y(), 1.);
        assert_eq!(point.z(), None);

        let mut point = Point3D(0., 1., 2.);
        point.set_x(10.);
        point.set_y(20.);
        point.set_z(30.);
        assert_eq!(point.x(), 10.);
        assert_eq!(point.y(), 20.);
        assert_eq!(point.z(), Some(30.));
        point.set_xy(40., 50.);
        assert_eq!(point.x(), 40.);
        assert_eq!(point.y(), 50.);
        assert_eq!(point.z(), Some(30.));
        point.set_xyz(60., 70., 80.);
        assert_eq!(point.x(), 60.);
        assert_eq!(point.y(), 70.);
        assert_eq!(point.z(), Some(80.));

        let point: Point3D = (&PointOrPoint3D(0., 1., Some(2.))).into();
        assert_eq!(point.x(), 0.);
        assert_eq!(point.y(), 1.);
        assert_eq!(point.z(), Some(2.));

        let point: Point3D = (&Point(0., 1.)).into();
        assert_eq!(point.x(), 0.);
        assert_eq!(point.y(), 1.);
        assert_eq!(point.z(), Some(0.));

        let mut point = PointOrPoint3D(0., 1., None);
        point.set_x(10.);
        point.set_y(20.);
        assert_eq!(point.x(), 10.);
        assert_eq!(point.y(), 20.);
        assert_eq!(point.z(), None);
        point.set_xy(30., 40.);
        assert_eq!(point.x(), 30.);
        assert_eq!(point.y(), 40.);
        assert_eq!(point.z(), None);

        let point: PointOrPoint3D = (&Point(0., 1.)).into();
        assert_eq!(point.x(), 0.);
        assert_eq!(point.y(), 1.);
        assert_eq!(point.z(), None);

        let point: PointOrPoint3D = (&Point3D(0., 1., 2.)).into();
        assert_eq!(point.x(), 0.);
        assert_eq!(point.y(), 1.);
        assert_eq!(point.z(), Some(2.));

        let mut point = PointOrPoint3D(0., 1., Some(2.));
        point.set_x(10.);
        point.set_y(20.);
        point.set_z(30.);
        assert_eq!(point.x(), 10.);
        assert_eq!(point.y(), 20.);
        assert_eq!(point.z(), Some(30.));
        point.set_xy(40., 50.);
        assert_eq!(point.x(), 40.);
        assert_eq!(point.y(), 50.);
        assert_eq!(point.z(), Some(30.));
        point.set_xyz(60., 70., 80.);
        assert_eq!(point.x(), 60.);
        assert_eq!(point.y(), 70.);
        assert_eq!(point.z(), Some(80.));
    }

    #[test]
    fn point_trait_new() {
        let point: Point = NewXY::new_xy(0., 1.);
        assert_eq!(point.x(), 0.);
        assert_eq!(point.y(), 1.);
        assert_eq!(point.z(), None);

        let point: Point3D = NewXY::new_xy(0., 1.);
        assert_eq!(point.x(), 0.);
        assert_eq!(point.y(), 1.);
        assert_eq!(point.z(), Some(0.));

        let point: Point3D = NewXYZ::new_xyz(0., 1., 2.);
        assert_eq!(point.x(), 0.);
        assert_eq!(point.y(), 1.);
        assert_eq!(point.z(), Some(2.));

        let point: PointOrPoint3D = NewXY::new_xy(0., 1.);
        assert_eq!(point.x(), 0.);
        assert_eq!(point.y(), 1.);
        assert_eq!(point.z(), None);

        let point: PointOrPoint3D = NewXYZ::new_xyz(0., 1., 2.);
        assert_eq!(point.x(), 0.);
        assert_eq!(point.y(), 1.);
        assert_eq!(point.z(), Some(2.));
    }

    #[test]
    fn to_m_geometry_points() {
        #[derive(Debug, Default, Clone, PartialEq, Copy)]
        struct TestA {
            x: f64,
        }
        impl From<TestA> for MValue {
            fn from(value: TestA) -> Self {
                let mut res = MValue::new();
                res.insert("x".into(), value.x.into());
                res
            }
        }
        impl From<MValue> for TestA {
            fn from(value: MValue) -> Self {
                TestA { x: value.get("x").unwrap().to_prim().unwrap().to_f64().unwrap() }
            }
        }
        impl From<&MValue> for TestA {
            fn from(value: &MValue) -> Self {
                TestA { x: value.get("x").unwrap().to_prim().unwrap().to_f64().unwrap() }
            }
        }
        impl MValueCompatible for TestA {}

        let geo: VectorGeometry<TestA> =
            VectorGeometry::new_point(VectorPoint::new_xy(0.5, 0.75, Some(TestA { x: 1.0 })), None);

        let geo_m = geo.to_m_geometry();
        assert_eq!(
            geo_m,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: VectorPoint::new_xy(
                    0.5,
                    0.75,
                    Some(MValue::from([("x".into(), 1.0.into())]))
                ),
                bbox: None,
                is_3d: false,
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_m_geometry_multipoints() {
        #[derive(Debug, Default, Clone, PartialEq, Copy)]
        struct TestA {
            x: f64,
        }
        impl From<TestA> for MValue {
            fn from(value: TestA) -> Self {
                let mut res = MValue::new();
                res.insert("x".into(), value.x.into());
                res
            }
        }
        impl From<MValue> for TestA {
            fn from(value: MValue) -> Self {
                TestA { x: value.get("x").unwrap().to_prim().unwrap().to_f64().unwrap() }
            }
        }
        impl From<&MValue> for TestA {
            fn from(value: &MValue) -> Self {
                TestA { x: value.get("x").unwrap().to_prim().unwrap().to_f64().unwrap() }
            }
        }
        impl MValueCompatible for TestA {}

        let geo: VectorGeometry<TestA> = VectorGeometry::new_multipoint(
            vec![
                VectorPoint::new_xy(0.5, 0.75, Some(TestA { x: 1.0 })),
                VectorPoint::new_xy(-0.5, -0.75, Some(TestA { x: -1.0 })),
            ],
            None,
        );

        let geo_m = geo.to_m_geometry();
        assert_eq!(
            geo_m,
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![
                    VectorPoint::new_xy(0.5, 0.75, Some(MValue::from([("x".into(), 1.0.into())]))),
                    VectorPoint::new_xy(
                        -0.5,
                        -0.75,
                        Some(MValue::from([("x".into(), (-1.0_f64).into())]))
                    )
                ],
                bbox: None,
                is_3d: false,
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_m_geometry_linestring() {
        #[derive(Debug, Default, Clone, PartialEq, Copy)]
        struct TestA {
            x: f64,
        }
        impl From<TestA> for MValue {
            fn from(value: TestA) -> Self {
                let mut res = MValue::new();
                res.insert("x".into(), value.x.into());
                res
            }
        }
        impl From<MValue> for TestA {
            fn from(value: MValue) -> Self {
                TestA { x: value.get("x").unwrap().to_prim().unwrap().to_f64().unwrap() }
            }
        }
        impl From<&MValue> for TestA {
            fn from(value: &MValue) -> Self {
                TestA { x: value.get("x").unwrap().to_prim().unwrap().to_f64().unwrap() }
            }
        }
        impl MValueCompatible for TestA {}

        let geo: VectorGeometry<TestA> = VectorGeometry::new_linestring(
            vec![
                VectorPoint::new_xy(0.5, 0.75, Some(TestA { x: 1.0 })),
                VectorPoint::new_xy(-0.5, -0.75, Some(TestA { x: -1.0 })),
            ],
            None,
        );

        let geo_m = geo.to_m_geometry();
        assert_eq!(
            geo_m,
            VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "LineString".into(),
                coordinates: vec![
                    VectorPoint::new_xy(0.5, 0.75, Some(MValue::from([("x".into(), 1.0.into())]))),
                    VectorPoint::new_xy(
                        -0.5,
                        -0.75,
                        Some(MValue::from([("x".into(), (-1.0_f64).into())]))
                    )
                ],
                bbox: None,
                is_3d: false,
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_m_geometry_multilinestring() {
        #[derive(Debug, Default, Clone, PartialEq, Copy)]
        struct TestA {
            x: f64,
        }
        impl From<TestA> for MValue {
            fn from(value: TestA) -> Self {
                let mut res = MValue::new();
                res.insert("x".into(), value.x.into());
                res
            }
        }
        impl From<MValue> for TestA {
            fn from(value: MValue) -> Self {
                TestA { x: value.get("x").unwrap().to_prim().unwrap().to_f64().unwrap() }
            }
        }
        impl From<&MValue> for TestA {
            fn from(value: &MValue) -> Self {
                TestA { x: value.get("x").unwrap().to_prim().unwrap().to_f64().unwrap() }
            }
        }
        impl MValueCompatible for TestA {}

        let geo: VectorGeometry<TestA> = VectorGeometry::new_multilinestring(
            vec![
                vec![
                    VectorPoint::new_xy(0.5, 0.75, Some(TestA { x: 1.0 })),
                    VectorPoint::new_xy(-0.5, -0.75, Some(TestA { x: -1.0 })),
                ],
                vec![
                    VectorPoint::new_xy(1.5, 1.75, Some(TestA { x: 2.0 })),
                    VectorPoint::new_xy(-1.5, -1.75, Some(TestA { x: -2.0 })),
                ],
            ],
            None,
        );

        let geo_m = geo.to_m_geometry();
        assert_eq!(
            geo_m,
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiLineString".into(),
                coordinates: vec![
                    vec![
                        VectorPoint::new_xy(
                            0.5,
                            0.75,
                            Some(MValue::from([("x".into(), 1.0.into())]))
                        ),
                        VectorPoint::new_xy(
                            -0.5,
                            -0.75,
                            Some(MValue::from([("x".into(), (-1.0_f64).into())]))
                        )
                    ],
                    vec![
                        VectorPoint::new_xy(
                            1.5,
                            1.75,
                            Some(MValue::from([("x".into(), 2.0.into())]))
                        ),
                        VectorPoint::new_xy(
                            -1.5,
                            -1.75,
                            Some(MValue::from([("x".into(), (-2.0_f64).into())]))
                        )
                    ]
                ],
                bbox: None,
                is_3d: false,
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_m_geometry_polygon() {
        #[derive(Debug, Default, Clone, PartialEq, Copy)]
        struct TestA {
            x: f64,
        }
        impl From<TestA> for MValue {
            fn from(value: TestA) -> Self {
                let mut res = MValue::new();
                res.insert("x".into(), value.x.into());
                res
            }
        }
        impl From<MValue> for TestA {
            fn from(value: MValue) -> Self {
                TestA { x: value.get("x").unwrap().to_prim().unwrap().to_f64().unwrap() }
            }
        }
        impl From<&MValue> for TestA {
            fn from(value: &MValue) -> Self {
                TestA { x: value.get("x").unwrap().to_prim().unwrap().to_f64().unwrap() }
            }
        }
        impl MValueCompatible for TestA {}

        let geo: VectorGeometry<TestA> = VectorGeometry::new_polygon(
            vec![
                vec![
                    VectorPoint::new_xy(0.5, 0.75, Some(TestA { x: 1.0 })),
                    VectorPoint::new_xy(-0.5, -0.75, Some(TestA { x: -1.0 })),
                ],
                vec![
                    VectorPoint::new_xy(1.5, 1.75, Some(TestA { x: 2.0 })),
                    VectorPoint::new_xy(-1.5, -1.75, Some(TestA { x: -2.0 })),
                ],
            ],
            None,
        );

        let geo_m = geo.to_m_geometry();
        assert_eq!(
            geo_m,
            VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: "Polygon".into(),
                coordinates: vec![
                    vec![
                        VectorPoint::new_xy(
                            0.5,
                            0.75,
                            Some(MValue::from([("x".into(), 1.0.into())]))
                        ),
                        VectorPoint::new_xy(
                            -0.5,
                            -0.75,
                            Some(MValue::from([("x".into(), (-1.0_f64).into())]))
                        )
                    ],
                    vec![
                        VectorPoint::new_xy(
                            1.5,
                            1.75,
                            Some(MValue::from([("x".into(), 2.0.into())]))
                        ),
                        VectorPoint::new_xy(
                            -1.5,
                            -1.75,
                            Some(MValue::from([("x".into(), (-2.0_f64).into())]))
                        )
                    ]
                ],
                bbox: None,
                is_3d: false,
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_m_geometry_multipolygon() {
        #[derive(Debug, Default, Clone, PartialEq, Copy)]
        struct TestA {
            x: f64,
        }
        impl From<TestA> for MValue {
            fn from(value: TestA) -> Self {
                let mut res = MValue::new();
                res.insert("x".into(), value.x.into());
                res
            }
        }
        impl From<MValue> for TestA {
            fn from(value: MValue) -> Self {
                TestA { x: value.get("x").unwrap().to_prim().unwrap().to_f64().unwrap() }
            }
        }
        impl From<&MValue> for TestA {
            fn from(value: &MValue) -> Self {
                TestA { x: value.get("x").unwrap().to_prim().unwrap().to_f64().unwrap() }
            }
        }
        impl MValueCompatible for TestA {}

        let geo: VectorGeometry<TestA> = VectorGeometry::new_multipolygon(
            vec![vec![
                vec![
                    VectorPoint::new_xy(0.5, 0.75, Some(TestA { x: 1.0 })),
                    VectorPoint::new_xy(-0.5, -0.75, Some(TestA { x: -1.0 })),
                ],
                vec![
                    VectorPoint::new_xy(1.5, 1.75, Some(TestA { x: 2.0 })),
                    VectorPoint::new_xy(-1.5, -1.75, Some(TestA { x: -2.0 })),
                ],
            ]],
            None,
        );

        let geo_m = geo.to_m_geometry();
        assert_eq!(
            geo_m,
            VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                coordinates: vec![vec![
                    vec![
                        VectorPoint::new_xy(
                            0.5,
                            0.75,
                            Some(MValue::from([("x".into(), 1.0.into())]))
                        ),
                        VectorPoint::new_xy(
                            -0.5,
                            -0.75,
                            Some(MValue::from([("x".into(), (-1.0_f64).into())]))
                        )
                    ],
                    vec![
                        VectorPoint::new_xy(
                            1.5,
                            1.75,
                            Some(MValue::from([("x".into(), 2.0.into())]))
                        ),
                        VectorPoint::new_xy(
                            -1.5,
                            -1.75,
                            Some(MValue::from([("x".into(), (-2.0_f64).into())]))
                        )
                    ]
                ]],
                bbox: None,
                is_3d: false,
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_point_eq_ord() {
        let a = Point(1.0, 2.0);
        let b = Point(1.0, 2.0);
        let c = Point(1.0, 3.0);
        let d = Point(0.5, 5.0);
        let nan = Point(f64::NAN, 1.0);

        assert_eq!(a, b);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
        assert_eq!(a.cmp(&b), Ordering::Equal);

        // x differs
        assert_eq!(a.cmp(&d), Ordering::Greater);
        assert_eq!(d.cmp(&a), Ordering::Less);

        // y differs
        assert_eq!(a.cmp(&c), Ordering::Less);
        assert_eq!(c.cmp(&a), Ordering::Greater);

        // NaN handled as Greater
        assert_eq!(nan.cmp(&a), Ordering::Greater);
        assert_eq!(a.cmp(&nan), Ordering::Greater);
    }

    #[test]
    fn test_point3d_eq_ord() {
        let a = Point3D(1.0, 2.0, 3.0);
        let b = Point3D(1.0, 2.0, 3.0);
        let xdiff = Point3D(0.5, 2.0, 3.0);
        let ydiff = Point3D(1.0, 5.0, 3.0);
        let zdiff = Point3D(1.0, 2.0, 4.0);
        let nan = Point3D(1.0, f64::NAN, 3.0);
        let nan_y = Point3D(1.0, f64::NAN, 3.0);
        let nan_z = Point3D(1.0, 2.0, f64::NAN);

        // equal
        assert_eq!(a, b);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
        assert_eq!(a.cmp(&b), Ordering::Equal);

        // x differs
        assert_eq!(a.cmp(&xdiff), Ordering::Greater);
        assert_eq!(xdiff.cmp(&a), Ordering::Less);

        // y differs
        assert_eq!(a.cmp(&ydiff), Ordering::Less);
        assert_eq!(ydiff.cmp(&a), Ordering::Greater);

        // z differs
        assert_eq!(a.cmp(&zdiff), Ordering::Less);
        assert_eq!(zdiff.cmp(&a), Ordering::Greater);

        // NaN treated as Equal in last fallback
        assert_eq!(nan.cmp(&a), Ordering::Greater);

        // triggers NaN handling in z comparison (None => Ordering::Equal)
        assert_eq!(nan_z.cmp(&a), Ordering::Equal);
        assert_eq!(a.cmp(&nan_z), Ordering::Equal);

        // NaN in y — handled earlier
        assert_eq!(nan_y.cmp(&a), Ordering::Greater);
    }

    #[test]
    fn test_point_or_point3d_eq_ord() {
        let a = PointOrPoint3D(1.0, 2.0, Some(3.0));
        let b = PointOrPoint3D(1.0, 2.0, Some(3.0));
        let xdiff = PointOrPoint3D(0.5, 2.0, Some(3.0));
        let ydiff = PointOrPoint3D(1.0, 3.0, Some(3.0));
        let zdiff = PointOrPoint3D(1.0, 2.0, Some(4.0));
        let none_z = PointOrPoint3D(1.0, 2.0, None);
        let nan = PointOrPoint3D(f64::NAN, 2.0, Some(3.0));
        let nan_z = PointOrPoint3D(1.0, 2.0, Some(f64::NAN));

        // equal
        assert_eq!(a, b);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
        assert_eq!(a.cmp(&b), Ordering::Equal);

        // x differs
        assert_eq!(a.cmp(&xdiff), Ordering::Greater);
        assert_eq!(xdiff.cmp(&a), Ordering::Less);

        // y differs
        assert_eq!(a.cmp(&ydiff), Ordering::Less);
        assert_eq!(ydiff.cmp(&a), Ordering::Greater);

        // z differs
        assert_eq!(a.cmp(&zdiff), Ordering::Less);
        assert_eq!(zdiff.cmp(&a), Ordering::Greater);

        // z = None (Option comparison)
        assert_eq!(a.cmp(&none_z), Ordering::Greater);
        assert_eq!(none_z.cmp(&a), Ordering::Less);

        // NaN in x
        assert_eq!(nan.cmp(&a), Ordering::Greater);
        assert_eq!(a.cmp(&nan), Ordering::Greater);

        // triggers None => Ordering::Equal in z comparison
        assert_eq!(nan_z.cmp(&a), Ordering::Equal);
        assert_eq!(a.cmp(&nan_z), Ordering::Equal);
    }
}
