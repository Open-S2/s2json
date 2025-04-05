extern crate alloc;

mod geometry;
mod impls;
mod map;
mod value;
mod vector_point;

#[cfg(test)]
mod tests {
    use alloc::vec;
    use s2json_core::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn face() {
        let face = Face::Face0;
        assert_eq!(u8::from(face), 0);
        let face = Face::Face1;
        assert_eq!(u8::from(face), 1);
        let face = Face::Face2;
        assert_eq!(u8::from(face), 2);
        let face = Face::Face3;
        assert_eq!(u8::from(face), 3);
        let face = Face::Face4;
        assert_eq!(u8::from(face), 4);
        let face = Face::Face5;
        assert_eq!(u8::from(face), 5);

        assert_eq!(Face::Face0, Face::from(0));
        assert_eq!(Face::Face1, Face::from(1));
        assert_eq!(Face::Face2, Face::from(2));
        assert_eq!(Face::Face3, Face::from(3));
        assert_eq!(Face::Face4, Face::from(4));
        assert_eq!(Face::Face5, Face::from(5));
    }

    #[test]
    fn defaults() {
        let f: Feature = Default::default();
        assert_eq!(f._type, "Feature".into());
        assert_eq!(f.id, None);
        assert_eq!(f.properties, Properties::default());
        assert_eq!(f.geometry, Geometry::default());
        assert_eq!(f.metadata, None);

        let f: VectorFeature = Default::default();
        assert_eq!(f._type, "VectorFeature".into());
        assert_eq!(f.id, None);
        assert_eq!(f.face, 0.into());
        assert_eq!(f.properties, Properties::default());
        assert_eq!(f.geometry, VectorGeometry::default());
        assert_eq!(f.metadata, None);
    }

    #[test]
    fn feature_collection_new() {
        let mut attributions = Attributions::new();
        attributions.insert("Open S2".into(), "https://opens2.com/legal/data".into());
        let mut fc = FeatureCollection::<()>::new(Some(attributions.clone()));
        assert_eq!(fc._type, FeatureCollectionType::FeatureCollection);
        assert_eq!(fc.features.len(), 0);
        assert_eq!(fc.attributions, Some(attributions.clone()));
        // update_bbox
        fc.update_bbox(BBox::new(5., -2., 35., 2.2));
        assert_eq!(fc.bbox, Some(BBox::new(5., -2., 35., 2.2)));

        let string = serde_json::to_string(&fc).unwrap();
        assert_eq!(string, "{\"type\":\"FeatureCollection\",\"features\":[],\"attributions\":{\"Open S2\":\"https://opens2.com/legal/data\"},\"bbox\":[5.0,-2.0,35.0,2.2]}");
        let back_to_fc: FeatureCollection = serde_json::from_str(&string).unwrap();
        assert_eq!(back_to_fc, fc);
    }

    #[test]
    fn s2_feature_collection_new() {
        let mut attributions = Attributions::new();
        attributions.insert("Open S2".into(), "https://opens2.com/legal/data".into());
        let mut fc = S2FeatureCollection::new(Some(attributions.clone()));
        assert_eq!(fc._type, S2FeatureCollectionType::S2FeatureCollection);
        assert_eq!(fc.features.len(), 0);
        assert_eq!(fc.attributions, Some(attributions.clone()));
        // update_bbox
        fc.update_bbox(BBox::new(5., -2., 35., 2.2));
        assert_eq!(fc.bbox, Some(BBox::new(5., -2., 35., 2.2)));
        // add face
        fc.add_face(0.into());
        fc.add_face(3.into());
        assert_eq!(fc.faces, vec![0.into(), 3.into()]);

        let string = serde_json::to_string(&fc).unwrap();
        assert_eq!(string, "{\"type\":\"S2FeatureCollection\",\"features\":[],\"faces\":[0,3],\"attributions\":{\"Open S2\":\"https://opens2.com/legal/data\"},\"bbox\":[5.0,-2.0,35.0,2.2]}");
        let back_to_fc: S2FeatureCollection = serde_json::from_str(&string).unwrap();
        assert_eq!(back_to_fc, fc);
    }

    #[test]
    fn feature_new() {
        let fc: Feature = Feature::new(
            Some(22),
            Properties::new(),
            Geometry::Point(PointGeometry {
                _type: "Point".into(),
                coordinates: Point(0.0, 0.0),
                m_values: None,
                bbox: None,
            }),
            None,
        );
        assert_eq!(fc.id, Some(22));
        assert_eq!(fc._type, "Feature".into());
        assert_eq!(
            fc.geometry,
            Geometry::Point(PointGeometry {
                _type: "Point".into(),
                coordinates: Point(0.0, 0.0),
                m_values: None,
                bbox: None,
            })
        );
        assert_eq!(fc.properties, Properties::new());
        assert_eq!(fc.metadata, None);
    }

    #[test]
    fn s2_feature_new() {
        let fc: VectorFeature = VectorFeature::new_wm(
            Some(55),
            Properties::new(),
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: VectorPoint { x: 0.0, y: 1.0, z: Some(3.), m: None, t: None },
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: None,
                indices: None,
                tessellation: None,
            }),
            None,
        );
        assert_eq!(fc.id, Some(55));
        assert_eq!(fc._type, "VectorFeature".into());
        assert_eq!(
            fc.geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: VectorPoint { x: 0.0, y: 1.0, z: Some(3.), m: None, t: None },
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: None,
                indices: None,
                tessellation: None,
            })
        );
        assert_eq!(fc.properties, Properties::new());
        assert_eq!(fc.metadata, None);
        assert_eq!(fc.face, 0.into());

        // S2

        #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
        struct MetaTest {
            name: String,
            value: String,
        }

        let fc = VectorFeature::<MetaTest>::new_s2(
            Some(55),
            3.into(),
            Properties::new(),
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: VectorPoint { x: 0.0, y: 1.0, z: Some(3.), m: None, t: None },
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: None,
                indices: None,
                tessellation: None,
            }),
            Some(MetaTest { name: "test".into(), value: "value".into() }),
        );
        assert_eq!(fc.id, Some(55));
        assert_eq!(fc._type, "S2Feature".into());
        assert_eq!(
            fc.geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: VectorPoint { x: 0.0, y: 1.0, z: Some(3.), m: None, t: None },
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: None,
                indices: None,
                tessellation: None,
            })
        );
        assert_eq!(fc.properties, Properties::new());
        assert_eq!(fc.metadata, Some(MetaTest { name: "test".into(), value: "value".into() }));
        assert_eq!(fc.face, 3.into());

        // TODO: BRING THIS BACK
        // let fc_to_str = serde_json::to_string(&fc).unwrap();
        // assert_eq!(
        //     fc_to_str,
        //     "{\"type\":\"S2Feature\",\"id\":55,\"face\":3,\"properties\":{},\"geometry\":{\"type\"\
        //      :\"Point\",\"is3D\":true,\"coordinates\":{\"x\":0.0,\"y\":1.0,\"z\":3.0}},\"metadata\":\
        //      ":{\"name\":\"test\",\"value\":\"value\"}}"
        // );

        // from_vector_feature

        let new_geo = VectorGeometry::Point(VectorPointGeometry {
            _type: "Point".into(),
            coordinates: VectorPoint { x: 5.0, y: 4.0, z: Some(-3.), m: None, t: None },
            bbox: None,
            is_3d: true,
            offset: None,
            vec_bbox: None,
            indices: None,
            tessellation: None,
        });
        let fc_clone_new_geometry =
            VectorFeature::<MetaTest>::from_vector_feature(&fc, Some(new_geo.clone()));

        assert_eq!(fc_clone_new_geometry.geometry, new_geo);
    }

    #[test]
    fn parse_feature_multipoint() {
        let json_string = r#"{
            "type": "Feature",
            "properties": {},
            "geometry": {
                "type": "MultiPoint",
                "coordinates": [
                    [-13.292352825505162, 54.34883408204476],
                    [36.83102287804303, 59.56941785818924],
                    [50.34083898563978, 16.040052775278994],
                    [76.38149901912357, 35.155968522292056]
                ]
            }
        }"#;

        let feature: Feature = serde_json::from_str(json_string).unwrap();
        assert_eq!(feature._type, "Feature".into());
        assert_eq!(
            feature.geometry,
            Geometry::MultiPoint(MultiPointGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![
                    Point(-13.292352825505162, 54.34883408204476),
                    Point(36.83102287804303, 59.56941785818924),
                    Point(50.34083898563978, 16.040052775278994),
                    Point(76.38149901912357, 35.155968522292056),
                ],
                ..Default::default()
            })
        );

        let back_to_str = serde_json::to_string(&feature).unwrap();
        assert_eq!(
            back_to_str,
            "{\"type\":\"Feature\",\"properties\":{},\"geometry\":{\"type\":\"MultiPoint\",\"\
             coordinates\":[[-13.292352825505162,54.34883408204476],[36.83102287804303,59.\
             56941785818924],[50.34083898563978,16.040052775278994],[76.38149901912357,35.\
             155968522292056]]}}"
        );
    }

    #[test]
    fn parse_feature_linestring() {
        let json_string = r#"{
            "type": "Feature",
            "properties": {},
            "geometry": {
                "type": "LineString",
                "coordinates": [
                    [-13.292352825505162, 54.34883408204476],
                    [36.83102287804303, 59.56941785818924],
                    [50.34083898563978, 16.040052775278994],
                    [76.38149901912357, 35.155968522292056]
                ]
            }
        }"#;

        let feature: Feature = serde_json::from_str(json_string).unwrap();
        assert_eq!(feature._type, "Feature".into());
        assert_eq!(
            feature.geometry,
            Geometry::LineString(LineStringGeometry {
                _type: "LineString".into(),
                coordinates: vec![
                    Point(-13.292352825505162, 54.34883408204476),
                    Point(36.83102287804303, 59.56941785818924),
                    Point(50.34083898563978, 16.040052775278994),
                    Point(76.38149901912357, 35.155968522292056),
                ],
                ..Default::default()
            })
        );

        let back_to_str = serde_json::to_string(&feature).unwrap();
        assert_eq!(
            back_to_str,
            "{\"type\":\"Feature\",\"properties\":{},\"geometry\":{\"type\":\"LineString\",\"\
             coordinates\":[[-13.292352825505162,54.34883408204476],[36.83102287804303,59.\
             56941785818924],[50.34083898563978,16.040052775278994],[76.38149901912357,35.\
             155968522292056]]}}"
        );
    }

    #[test]
    fn parse_vector_feature_linestring() {
        let json_string = r#"{
            "type": "VectorFeature",
            "face": 0,
            "properties": {},
            "geometry": {
                "type": "LineString",
                "is3D": false,
                "coordinates": [
                    { "x": -13.292352825505162, "y": 54.34883408204476 },
                    { "x": 36.83102287804303, "y": 59.56941785818924 },
                    { "x": 50.34083898563978, "y": 16.040052775278994 },
                    { "x": 76.38149901912357, "y": 35.155968522292056 }
                ]
            }
        }"#;

        let feature: VectorFeature = serde_json::from_str(json_string).unwrap();
        assert_eq!(feature._type, "VectorFeature".into());
        let geometry = feature.geometry;
        assert_eq!(
            geometry,
            VectorGeometry::LineString(VectorLineStringGeometry {
                _type: VectorGeometryType::LineString,
                is_3d: false,
                coordinates: vec![
                    VectorPoint::from_xy(-13.292352825505162, 54.34883408204476),
                    VectorPoint::from_xy(36.83102287804303, 59.56941785818924),
                    VectorPoint::from_xy(50.34083898563978, 16.040052775278994),
                    VectorPoint::from_xy(76.38149901912357, 35.155968522292056),
                ],
                ..Default::default()
            })
        )
    }

    #[test]
    #[should_panic(expected = "Invalid vector geometry type: LinesString")]
    fn vector_geometry_type_from() {
        let str_bad = "LinesString";
        let _ = VectorGeometryType::from(str_bad);
    }

    #[test]
    fn parse_vector_feature_multipoint() {
        let json_string = r#"{
            "type": "VectorFeature",
            "face": 0,
            "properties": {},
            "geometry": {
                "type": "MultiPoint",
                "is3D": false,
                "coordinates": [
                    { "x": -13.292352825505162, "y": 54.34883408204476 },
                    { "x": 36.83102287804303, "y": 59.56941785818924 },
                    { "x": 50.34083898563978, "y": 16.040052775278994 },
                    { "x": 76.38149901912357, "y": 35.155968522292056 }
                ]
            }
        }"#;

        let feature: VectorFeature = serde_json::from_str(json_string).unwrap();
        assert_eq!(feature._type, "VectorFeature".into());
        let geometry = feature.geometry;
        assert_eq!(
            geometry,
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: VectorGeometryType::MultiPoint,
                is_3d: false,
                coordinates: vec![
                    VectorPoint::from_xy(-13.292352825505162, 54.34883408204476),
                    VectorPoint::from_xy(36.83102287804303, 59.56941785818924),
                    VectorPoint::from_xy(50.34083898563978, 16.040052775278994),
                    VectorPoint::from_xy(76.38149901912357, 35.155968522292056),
                ],
                ..Default::default()
            })
        )
    }

    #[test]
    #[should_panic(expected = "Invalid geometry type: MultiLinesString")]
    fn vector_geometry_type_from_bad() {
        let str_bad = "MultiLinesString";
        let _ = GeometryType::from(str_bad);
    }

    #[test]
    fn serde_face() {
        let face_0 = Face::Face0;
        let serialized = serde_json::to_string(&face_0).unwrap();
        assert_eq!(serialized, "0");
        let deserialize = serde_json::from_str::<Face>(&serialized).unwrap();
        assert_eq!(deserialize, Face::Face0);

        let face_1 = Face::Face1;
        let serialized = serde_json::to_string(&face_1).unwrap();
        assert_eq!(serialized, "1");
        let deserialize = serde_json::from_str::<Face>(&serialized).unwrap();
        assert_eq!(deserialize, Face::Face1);

        let face_2 = Face::Face2;
        let serialized = serde_json::to_string(&face_2).unwrap();
        assert_eq!(serialized, "2");
        let deserialize = serde_json::from_str::<Face>(&serialized).unwrap();
        assert_eq!(deserialize, Face::Face2);

        let face_3 = Face::Face3;
        let serialized = serde_json::to_string(&face_3).unwrap();
        assert_eq!(serialized, "3");
        let deserialize = serde_json::from_str::<Face>(&serialized).unwrap();
        assert_eq!(deserialize, Face::Face3);

        let face_4 = Face::Face4;
        let serialized = serde_json::to_string(&face_4).unwrap();
        assert_eq!(serialized, "4");
        let deserialize = serde_json::from_str::<Face>(&serialized).unwrap();
        assert_eq!(deserialize, Face::Face4);

        let face_5 = Face::Face5;
        let serialized = serde_json::to_string(&face_5).unwrap();
        assert_eq!(serialized, "5");
        let deserialize = serde_json::from_str::<Face>(&serialized).unwrap();
        assert_eq!(deserialize, Face::Face5);
    }

    #[test]
    #[should_panic(expected = "Invalid face value")]
    fn serde_face_err() {
        let _ = serde_json::from_str::<Face>("6").unwrap();
    }

    #[test]
    fn to_m() {
        #[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
        struct MetaTest {
            name: String,
            value: String,
        }

        let fc = VectorFeature::<MetaTest>::new_s2(
            Some(55),
            3.into(),
            Properties::new(),
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: VectorPoint { x: 0.0, y: 1.0, z: Some(3.), m: None, t: None },
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: None,
                indices: None,
                tessellation: None,
            }),
            Some(MetaTest { name: "test".into(), value: "value".into() }),
        );

        let fc_m = fc.to_m_vector_feature(|meta| {
            let meta = meta.unwrap().clone();
            Some(MValue::from([("a".into(), meta.name.into())]))
        });

        assert_eq!(fc_m.metadata, Some(MValue::from([("a".into(), "test".into())])));
    }
}
