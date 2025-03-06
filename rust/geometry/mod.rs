/// BBox and BBox3D shapes and utilities
pub mod bbox;
/// Primitive geometry types (used by GeoJSON spec)
pub mod primitive;
/// Vector geometry types (used by the s2json spec for both WGS84 and S2Geometry)
pub mod vector;

pub use bbox::*;
pub use primitive::*;
pub use vector::*;

use serde::{Deserialize, Serialize};

use crate::{Face, MValue, MValueCompatible};

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
pub struct STPoint<M: MValueCompatible = MValue> {
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

#[cfg(test)]
mod tests {
    use crate::*;

    use super::*;
    use alloc::string::ToString;
    use alloc::vec;
    use serde_json::json;

    #[test]
    fn test_vector_offset() {
        let offset = VectorOffsets::default();
        assert_eq!(offset, VectorOffsets::LineOffset(0.0));
        let offset: VectorOffsets = Default::default();
        assert_eq!(offset, VectorOffsets::LineOffset(0.0));
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
            BBox {
                left: f64::INFINITY,
                bottom: f64::INFINITY,
                right: -f64::INFINITY,
                top: -f64::INFINITY
            }
        );

        let default_bbox_2 = BBOX::default();
        assert_eq!(
            default_bbox_2,
            BBOX::BBox(BBox {
                left: f64::INFINITY,
                bottom: f64::INFINITY,
                right: -f64::INFINITY,
                top: -f64::INFINITY
            })
        );
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
        assert!(bbox.point_overlap(VectorPoint::<MValue>::new(0.5, 0.5, None, None)));
        assert!(!bbox.point_overlap(VectorPoint::<MValue>::new(2.0, 2.0, None, None)));
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
    }

    #[test]
    fn test_bbox_from_multilinestring() {
        let bbox = BBox::from_multi_linestring(&vec![vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(1., 1.5, None, None),
        ]]);
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.5 });
    }

    #[test]
    fn test_bbox_from_polygon() {
        let bbox = BBox::from_polygon(&vec![vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(2., 1.5, None, None),
        ]]);
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 2.0, top: 1.5 });
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
                left: f64::INFINITY,
                bottom: f64::INFINITY,
                right: -f64::INFINITY,
                top: -f64::INFINITY,
                near: f64::INFINITY,
                far: -f64::INFINITY
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
        assert!(bbox.point_overlap(VectorPoint::<MValue>::new(0.5, 0.5, None, None)));
        assert!(!bbox.point_overlap(VectorPoint::<MValue>::new(2.0, 2.0, None, None)));
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
            BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 0.0,
                top: 0.0,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
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
            BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 1.0,
                top: 1.5,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
    }

    #[test]
    fn test_bbox_3_d_from_multilinestring() {
        let bbox = BBox3D::from_multi_linestring(&vec![vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(1., 1.5, None, None),
        ]]);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 1.0,
                top: 1.5,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
    }

    #[test]
    fn test_bbox_3_d_from_polygon() {
        let bbox = BBox3D::from_polygon(&vec![vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(2., 1.5, None, None),
        ]]);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 2.0,
                top: 1.5,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
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
            BBox3D {
                left: -1.0,
                bottom: 0.0,
                right: 2.0,
                top: 3.5,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
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
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
    }

    #[test]
    fn test_bbox_3_d_from_st_uv() {
        let bbox = BBox3D::from_st_zoom(0., 0., 0);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 1.,
                top: 1.,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );

        let bbox = BBox3D::from_st_zoom(1., 0., 1);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.5,
                bottom: 0.0,
                right: 1.,
                top: 0.5,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );

        let bbox = BBox3D::from_st_zoom(2., 0., 2);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.5,
                bottom: 0.0,
                right: 0.75,
                top: 0.25,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );

        let bbox = BBox3D::from_uv_zoom(0., 0., 0);
        assert_eq!(
            bbox,
            BBox3D {
                left: -1.0,
                bottom: -1.0,
                right: 1.,
                top: 1.,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );

        let bbox = BBox3D::from_uv_zoom(1., 0., 1);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.,
                bottom: -1.0,
                right: 1.,
                top: 0.,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );

        let bbox = BBox3D::from_uv_zoom(2., 0., 2);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.,
                bottom: -1.0,
                right: 0.5,
                top: -0.5,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
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
            tesselation: None,
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
                tesselation: None,
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
                tesselation: None,
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
                tesselation: None,
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
            tesselation: None,
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
                tesselation: None,
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
            if let WMFeature::Feature(first_feature) = &fc.features[0] {
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
                coordinates: point,
                bbox: Some(bbox),
                is_3d: false,
                ..Default::default()
            })
        );
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
                coordinates: point,
                bbox: Some(bbox),
                is_3d: true,
                ..Default::default()
            })
        );
    }

    #[test]
    fn vector_geometry_new_multipoint() {
        let multipoint = vec![VectorPoint::from_xy(0.5, 0.75), VectorPoint::from_xy(1.75, 2.5)];
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let geometry = VectorGeometry::new_multipoint(multipoint.clone(), Some(bbox));

        assert_eq!(
            geometry,
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                coordinates: multipoint,
                bbox: Some(bbox),
                is_3d: false,
                ..Default::default()
            })
        );
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
                coordinates: multipoint,
                bbox: Some(bbox),
                is_3d: true,
                ..Default::default()
            })
        );
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
                coordinates: line_string,
                bbox: Some(bbox),
                is_3d: false,
                ..Default::default()
            })
        );
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
                coordinates: linestring,
                bbox: Some(bbox),
                is_3d: true,
                ..Default::default()
            })
        );
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
                coordinates: multi_line_string,
                bbox: Some(bbox),
                is_3d: false,
                ..Default::default()
            })
        );
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
                coordinates: multi_line_string,
                bbox: Some(bbox),
                is_3d: true,
                ..Default::default()
            })
        );
    }

    #[test]
    fn vector_geometry_new_polygon() {
        let polygon = vec![
            vec![VectorPoint::from_xy(0.5, 0.75), VectorPoint::from_xy(1.75, 2.5)],
            vec![VectorPoint::from_xy(1.5, 2.75), VectorPoint::from_xy(3.75, 4.5)],
        ];
        let bbox = BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        let geometry = VectorGeometry::new_polygon(polygon.clone(), Some(bbox));

        assert_eq!(
            geometry,
            VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: "Polygon".into(),
                coordinates: polygon,
                bbox: Some(bbox),
                is_3d: false,
                ..Default::default()
            })
        );
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
                coordinates: polygon_3d,
                bbox: Some(bbox),
                is_3d: true,
                ..Default::default()
            })
        );
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
        let geometry = VectorGeometry::new_multipolygon(multipolygon.clone(), Some(bbox));

        assert_eq!(
            geometry,
            VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                coordinates: multipolygon,
                bbox: Some(bbox),
                is_3d: false,
                ..Default::default()
            })
        );
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
                coordinates: multipolygon,
                bbox: Some(bbox),
                is_3d: true,
                ..Default::default()
            })
        );
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
}
