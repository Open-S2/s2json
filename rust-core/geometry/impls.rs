use crate::*;

// We have to manually implement the Deserialize trait to fix a compilation error
#[doc(hidden)]
#[allow(unused_extern_crates, clippy::useless_attribute)]
extern crate serde as _serde;
#[automatically_derived]
// #[coverage(off)]
impl<'de, M: Clone + Default> Deserialize<'de> for Geometry<M>
where
    M: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: _serde::Deserializer<'de>,
    {
        // 1. Deserialize into an intermediate Value.
        let value = serde_json::Value::deserialize(deserializer)?;

        // 2. Attempt to deserialize from the `value` into each possible variant.
        // We use serde_json::from_value(value.clone()) to create owned instances
        // and avoid the lifetime error where a borrow outlives the owned 'value'.
        if let Ok(geom) = PointGeometry::<M>::deserialize(value.clone()) {
            return Ok(Geometry::Point(geom));
        }

        // Attempt to deserialize as MultiPoint then check if it's actually a LineString
        if let Ok(multipoint) = MultiPointGeometry::<M>::deserialize(value.clone()) {
            if multipoint._type == GeometryType::LineString {
                // If the type matches, we prioritize the LineString deserialization.
                if let Ok(linestring) = LineStringGeometry::<M>::deserialize(value.clone()) {
                    return Ok(Geometry::LineString(linestring));
                }
            }
            // Otherwise, or if LineString parsing failed, we fall back to the valid MultiPoint.
            return Ok(Geometry::MultiPoint(multipoint));
        }

        // Attempt as MultiLineString, then check for Polygon
        if let Ok(multilinestring) = MultiLineStringGeometry::<M>::deserialize(value.clone()) {
            if multilinestring._type == GeometryType::Polygon {
                if let Ok(polygon) = PolygonGeometry::<M>::deserialize(value.clone()) {
                    return Ok(Geometry::Polygon(polygon));
                }
            }
            return Ok(Geometry::MultiLineString(multilinestring));
        }

        if let Ok(geom) = MultiPolygonGeometry::<M>::deserialize(value.clone()) {
            return Ok(Geometry::MultiPolygon(geom));
        }
        if let Ok(geom) = Point3DGeometry::<M>::deserialize(value.clone()) {
            return Ok(Geometry::Point3D(geom));
        }

        // Attempt as MultiPoint3D, then check for LineString3D
        if let Ok(multipoint3d) = MultiPoint3DGeometry::<M>::deserialize(value.clone()) {
            if multipoint3d._type == GeometryType::LineString3D {
                if let Ok(linestring3d) = LineString3DGeometry::<M>::deserialize(value.clone()) {
                    return Ok(Geometry::LineString3D(linestring3d));
                }
            }
            return Ok(Geometry::MultiPoint3D(multipoint3d));
        }

        // Attempt as MultiLineString3D, then check for Polygon3D
        if let Ok(multilinestring3d) = MultiLineString3DGeometry::<M>::deserialize(value.clone()) {
            if multilinestring3d._type == GeometryType::Polygon3D {
                if let Ok(polygon3d) = Polygon3DGeometry::<M>::deserialize(value.clone()) {
                    return Ok(Geometry::Polygon3D(polygon3d));
                }
            }
            return Ok(Geometry::MultiLineString3D(multilinestring3d));
        }

        if let Ok(geom) = MultiPolygon3DGeometry::<M>::deserialize(value.clone()) {
            return Ok(Geometry::MultiPolygon3D(geom));
        }

        Err(_serde::de::Error::custom("data did not match any variant of untagged enum Geometry"))
    }
}

#[doc(hidden)]
#[allow(unused_extern_crates, clippy::useless_attribute)]
#[automatically_derived]
// #[coverage(off)]
impl<'de, M: Clone + Default> Deserialize<'de> for VectorGeometry<M>
where
    M: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: _serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if let Ok(geom) = VectorPointGeometry::<M>::deserialize(value.clone()) {
            return Ok(VectorGeometry::Point(geom));
        }

        // Attempt to deserialize as MultiPoint, then check for LineString
        if let Ok(multipoint) = VectorMultiPointGeometry::<M>::deserialize(value.clone()) {
            if multipoint._type == VectorGeometryType::LineString {
                if let Ok(linestring) = VectorLineStringGeometry::<M>::deserialize(value.clone()) {
                    return Ok(VectorGeometry::LineString(linestring));
                }
            }
            return Ok(VectorGeometry::MultiPoint(multipoint));
        }

        // Attempt to deserialize as MultiLineString, then check for Polygon
        if let Ok(multilinestring) = VectorMultiLineStringGeometry::<M>::deserialize(value.clone())
        {
            if multilinestring._type == VectorGeometryType::Polygon {
                if let Ok(polygon) = VectorPolygonGeometry::<M>::deserialize(value.clone()) {
                    return Ok(VectorGeometry::Polygon(polygon));
                }
            }
            return Ok(VectorGeometry::MultiLineString(multilinestring));
        }

        if let Ok(geom) = VectorMultiPolygonGeometry::<M>::deserialize(value.clone()) {
            return Ok(VectorGeometry::MultiPolygon(geom));
        }

        Err(_serde::de::Error::custom(
            "data did not match any variant of untagged enum VectorGeometry",
        ))
    }
}
