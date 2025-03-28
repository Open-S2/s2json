use crate::*;

// We have to manually implement the Deserialize trait to fix a compilation error
#[doc(hidden)]
#[allow(unused_extern_crates, clippy::useless_attribute)]
extern crate serde as _serde;
#[automatically_derived]
#[coverage(off)]
impl<'de, M: Clone + Default> _serde::Deserialize<'de> for Geometry<M>
where
    M: _serde::Deserialize<'de>,
{
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        let __content =
            <_serde::__private::de::Content as _serde::Deserialize>::deserialize(__deserializer)?;
        let __deserializer =
            _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content);
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <PointGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::Point,
        ) {
            return _serde::__private::Ok(__ok);
        }
        // Attempt to deserialize as MultiPoint then check for LineString
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <MultiPointGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::MultiPoint,
        ) {
            // pull out the MultiPoint variant
            if let Geometry::MultiPoint(multipoint) = &__ok {
                if multipoint._type == GeometryType::LineString {
                    // If deserialization succeeds as MultiPoint, check if content is LineString
                    if let _serde::__private::Ok(__ok2) = _serde::__private::Result::map(
                        <LineStringGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
                        Geometry::LineString,
                    ) {
                        // If LineString is found, return LineString variant
                        return _serde::__private::Ok(__ok2);
                    }
                }
            }
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <MultiLineStringGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::MultiLineString,
        ) {
            // pull out the MultiLineString variant
            if let Geometry::MultiLineString(multilinestring) = &__ok {
                if multilinestring._type == GeometryType::Polygon {
                    if let _serde::__private::Ok(__ok2) = _serde::__private::Result::map(
                        <PolygonGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
                        Geometry::Polygon,
                    ) {
                        return _serde::__private::Ok(__ok2);
                    }
                }
            }
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <MultiPolygonGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::MultiPolygon,
        ) {
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <Point3DGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::Point3D,
        ) {
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <MultiPoint3DGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::MultiPoint3D,
        ) {
            // pull out the MultiPoint3D variant
            if let Geometry::MultiPoint3D(multipoint3d) = &__ok {
                if multipoint3d._type == GeometryType::LineString3D {
                    if let _serde::__private::Ok(__ok2) = _serde::__private::Result::map(
                        <LineString3DGeometry<M> as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        Geometry::LineString3D,
                    ) {
                        return _serde::__private::Ok(__ok2);
                    }
                }
            }
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <MultiLineString3DGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::MultiLineString3D,
        ) {
            // pull out the MultiLineString3D variant
            if let Geometry::MultiLineString3D(multilinestring3d) = &__ok {
                if multilinestring3d._type == GeometryType::Polygon3D {
                    if let _serde::__private::Ok(__ok2) = _serde::__private::Result::map(
                        <Polygon3DGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
                        Geometry::Polygon3D,
                    ) {
                        return _serde::__private::Ok(__ok2);
                    }
                }
            }
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <MultiPolygon3DGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::MultiPolygon3D,
        ) {
            return _serde::__private::Ok(__ok);
        }
        _serde::__private::Err(_serde::de::Error::custom(
            "data did not match any variant of untagged enum Geometry",
        ))
    }
}

#[doc(hidden)]
#[allow(unused_extern_crates, clippy::useless_attribute)]
#[automatically_derived]
#[coverage(off)]
impl<'de, M: Clone + Default> _serde::Deserialize<'de> for VectorGeometry<M>
where
    M: _serde::Deserialize<'de>,
{
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        let __content =
            <_serde::__private::de::Content as _serde::Deserialize>::deserialize(__deserializer)?;
        let __deserializer =
            _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content);
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <VectorPointGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            VectorGeometry::Point,
        ) {
            return _serde::__private::Ok(__ok);
        }
        // Attempt to deserialize as MultiPoint, then check for LineString
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <VectorMultiPointGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            VectorGeometry::MultiPoint,
        ) {
            // pull out the MultiPoint variant
            if let VectorGeometry::MultiPoint(multipoint) = &__ok {
                if multipoint._type == VectorGeometryType::LineString {
                    // If deserialization succeeds as MultiPoint, check if content is LineString
                    if let _serde::__private::Ok(__ok2) = _serde::__private::Result::map(
                        <VectorLineStringGeometry<M> as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        VectorGeometry::LineString,
                    ) {
                        // If LineString is found, return LineString variant
                        return _serde::__private::Ok(__ok2);
                    }
                }
            }
            return _serde::__private::Ok(__ok);
        }
        // Attempt to deserialize as MultiLineString, then check for Polygon
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <VectorMultiLineStringGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            VectorGeometry::MultiLineString,
        ) {
            // pull out the MultiLineString variant
            if let VectorGeometry::MultiLineString(multilinestring) = &__ok {
                if multilinestring._type == VectorGeometryType::Polygon {
                    // If deserialization succeeds as MultiLineString, check if content is Polygon
                    if let _serde::__private::Ok(__ok2) = _serde::__private::Result::map(
                        <VectorPolygonGeometry<M> as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        VectorGeometry::Polygon,
                    ) {
                        // If Polygon is found, return Polygon variant
                        return _serde::__private::Ok(__ok2);
                    }
                }
            }
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <VectorMultiPolygonGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            VectorGeometry::MultiPolygon,
        ) {
            return _serde::__private::Ok(__ok);
        }
        _serde::__private::Err(_serde::de::Error::custom(
            "data did not match any variant of untagged enum VectorGeometry",
        ))
    }
}
