use crate::{Face, LonLat, S2CellId, S2Feature, VectorFeature, VectorGeometry, VectorPoint};

impl From<&mut S2Feature> for VectorFeature {
    fn from(feature: &mut S2Feature) -> Self {
        let mut geometry = feature.geometry.clone();
        convert_geometry(feature.face, &mut geometry);
        VectorFeature::new(feature.id, feature.properties.clone(), geometry, feature.metadata)
    }
}

/// Underlying conversion mechanic to move S2Geometry to GeoJSON Geometry
fn convert_geometry(face: Face, geometry: &mut VectorGeometry) {
    match geometry {
        VectorGeometry::Point(point) => convert_geometry_point(face, &mut point.coordinates),
        VectorGeometry::MultiPoint(points) => {
            points.coordinates.iter_mut().for_each(|point| convert_geometry_point(face, point))
        }
        VectorGeometry::LineString(line) => {
            line.coordinates.iter_mut().for_each(|point| convert_geometry_point(face, point))
        }
        VectorGeometry::MultiLineString(lines) => lines
            .coordinates
            .iter_mut()
            .for_each(|line| line.iter_mut().for_each(|point| convert_geometry_point(face, point))),
        VectorGeometry::Polygon(polygon) => polygon
            .coordinates
            .iter_mut()
            .for_each(|line| line.iter_mut().for_each(|point| convert_geometry_point(face, point))),
        VectorGeometry::MultiPolygon(polygons) => {
            polygons.coordinates.iter_mut().for_each(|polygon| {
                polygon.iter_mut().for_each(|line| {
                    line.iter_mut().for_each(|point| convert_geometry_point(face, point))
                })
            })
        }
    }
}

/// Mutate an S2 Point to a GeoJSON Point
fn convert_geometry_point(face: Face, point: &mut VectorPoint) {
    let LonLat { lon, lat } = (&S2CellId::from_face_st(face.into(), point.x, point.y)).into();
    point.x = lon;
    point.y = lat;
}
