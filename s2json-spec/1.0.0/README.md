# S2JSON Specification 1.0.0

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in
this document are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

## 0. Conventions & Terminology

The ordering of the members of any JSON object defined in this document MUST be considered irrelevant, as specified by [RFC7159](https://www.ietf.org/rfc/rfc7159.txt).

Some examples use the combination of a JavaScript single-line comment (//) followed by an ellipsis (...) as placeholder notation for content deemed irrelevant by the authors.  These placeholders must of course be deleted or otherwise replaced, before attempting to validate the corresponding JSON code example.

Whitespace is used in the examples inside this document to help illustrate the data structures, but it is not required.  Unquoted whitespace is not significant in JSON.

WG: [World Geodetic System 1984 (WGS84)](https://gisgeography.com/wgs84-world-geodetic-system/)
S2: [S2 Geometry](http://s2geometry.io/about/overview)
LonLat: [Longitude, Latitude](https://en.wikipedia.org/wiki/Longitude_and_latitude)

## 1. Purpose

This document specifies a space-efficient encoding format for tiled geographic vector data. It is designed to be used in browsers or server-side applications for fast rendering or lookups of feature data.

## 2. File Format

Files are JSON (JavaScript Object Notation) files. Data is stored as utf-8 variable encoded strings. Some text in feature properties or m-values MAY contain [Unicode](https://en.wikipedia.org/wiki/Unicode) characters.

### 2.1. File Extension

The filename extension for Vector Tile files MAY be `.json`. For example, a file might be named `data.json`. However, while OPTIONAL, it is strongly RECOMMENDED that that WGS84 features be stored as `.geojson` or `.geojsonld`, and S2 Geometry features be stored as `.s2json` or `.s2jsonld`.

### 2.2. Data Storage

There are two ways in which data may be stored in a string and/or a file: Line delimited JSON and Single JSON Object.

#### 2.2.1 Line Delimited JSON

Each Line is a Valid JSON Value. Thus, a collection of `Feature` or `S2Feature` objects are stored individually on each line. Lines are separated by utf-8 newline characters, `\n` or `\r\n`.

Note that JSON allows encoding Unicode strings with only ASCII escape sequences, however those escapes will be hard to read when viewed in a text editor.

#### 2.2.2 Single JSON Object

This may be either a `Feature`, `S2Feature`, `FeatureCollection`, or `S2FeatureCollection` object.

## 3. Coordinate Reference Systems

This specification utilizes two primary coordinate reference systems (CRS): WGS84 and S2 Geometry. Each system offers distinct advantages for certain types of geographic data processing and visualization.

### 3.1. WGS84 (EPSG:4326)

WGS84, or the World Geodetic System 1984, is the global standard for Earth's geoid used in mapping and navigation. This geodetic datum is defined by the International Earth Rotation and Reference Systems Service and is widely employed for global positioning systems (GPS) and general geographic information system (GIS) applications.

In the context of this specification, WGS84 is used primarily for representing geographic features in a format compatible with widespread GIS tools and software. It defines coordinates in terms of latitude and longitude, which are expressed as decimal degrees.

Features using WGS84 are stored as `.geojson` or `.geojsonld` files, aligning with established practices in geospatial data interchange. This allows for seamless integration with existing software ecosystems and promotes interoperability.

Example of WGS84 coordinates for a point:

```json
{
  "type": "Feature",
  "geometry": {
    "type": "Point",
    "coordinates": [-123.3656, 48.4284]  // Longitude, Latitude
  },
  "properties": {
    "name": "Example Location"
  }
}
```

### 3.2. S2 (S2 Geometry)

S2 Geometry, developed by Google, is a spherical geometry system that projects the Earth’s surface onto a unit sphere, which is then subdivided into hierarchical cells. Each cell is identified by a unique identifier, facilitating efficient querying and data retrieval operations, particularly for large geographic datasets.

S2 Geometry is particularly advantageous for applications requiring rapid spatial searches and data scalability. It handles edge cases like the poles and the 180th meridian more gracefully than traditional latitude-longitude-based systems.

This specification leverages S2 for optimized spatial data processing and storage. Data using S2 Geometry are recommended to be stored as .s2json or .s2jsonld files to highlight their use of this specialized system and to differentiate them from traditional geojson files.

Example of an S2 Geometry-encoded feature:

```json
{
  "type": "S2Feature",
  "geometry": {
    "type": "Point",
    "coordinates": [0.1, 0.5]  // S2 cell ID as coordinates
  },
  "face": 2,
  "properties": {
    "name": "Example S2 Feature"
  }
}
```

## 4. S2 Face

In S2 Geometry, the world is represented as a unit sphere and divided into six faces using a cube -like projection. Each face of the cube MUST be identified by an integer index from 0 to 5 inclusive.

Example:

```json
{
  "type": "S2Feature",
  "properties": {
    "name": "Example Feature"
  },
  "geometry": {
    "type": "Point",
    "coordinates": [0.1, 0.5]
  },
  "face": 2
}
```

## 5. Values

### 5.1. Primitive Values

Primitive values include strings, numbers, booleans, and null.

Example:

```json
{
  "stringValue": "example",
  "numberValue": 42,
  "booleanValue": true,
  "nullValue": null
}
```

### 5.2. Array Values

Array values are ordered lists of values. Once you use an array, each item MUST be of the same type. Each item MUST be either a primitive value or an object whose keys are strings and values are primitive values.

Example:

```json
{
  "arrayValue": [1, 2, 3, 4, 5]
}
```

### 5.3. Object Values (nested values)

Object values are collections of key-value pairs, where values can be of any type, including nested objects.

Example:

```json
{
  "objectValue": {
    "key1": "value1",
    "key2": {
      "nestedKey": "nestedValue"
    }
  }
}
```

## 6. Properties

Properties are key-value pairs that store additional data about features. The key is a string and the value is any `Value` type: `primitive`, `array`, or `object`.

Example:

```json
{
  "properties": {
    "name": "Example Feature",
    "category": "example",
    "metadata": {
      "createdBy": "user123",
      "createdAt": "2023-01-01T00:00:00Z"
    }
  }
}
```

## 7. MValues

MValues provide additional attribute data for each coordinate in a geometry. They are OPTIONAL and follow the same structure as `properties`.

The length of mValues MUST match the length of the corresponding coordinates.

All mValues MUST have the same structure with the same key-value pairs.

Example:

```json
{
  "type": "LineString",
  "coordinates": [
    [100.0, 0.0],
    [101.0, 1.0]
  ],
  "mValues": [
    { "foo": "bar" },
    { "foo": "baz" }
  ]
}
```

## 8. FeatureCollections

FeatureCollections are containers that group multiple geographic features into a single object. This aggregation enhances the handling and processing of grouped features, applicable in scenarios ranging from data visualization to complex spatial analysis.

### 8.1. FeatureCollection

A `FeatureCollection` is defined strictly within the context of traditional geospatial data handling, using the WGS84 coordinate reference system. It does not include features defined in the S2 Geometry system; those are handled separately in an `S2FeatureCollection`.

### 8.1.1. FeatureCollection Type

The `type` property of a `FeatureCollection` is a fixed string identifier indicating the collection's format. For a standard `FeatureCollection`, the `type` MUST be set to `"FeatureCollection"`. This property is REQUIRED and specifies the nature of the JSON object as a container for multiple geographical features, ensuring compatibility with GeoJSON standards.

Example of the `type` property in a `FeatureCollection`:

```json
{
  "type": "FeatureCollection"
}
```

### 8.1.2. FeatureCollection Features

The `features` array is a REQUIRED part of a `FeatureCollection` and consists of individual Feature objects. Each Feature within the array MUST comply with the GeoJSON format specifications, containing at least a type, properties, and geometry field. This structure not only ensures compliance with broader geospatial data standards but also facilitates integration with various GIS tools and libraries.

Example of the `features` array in a `FeatureCollection`:

```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [-123.3656, 48.4284]
      },
      "properties": {
        "name": "Example Point"
      }
    }
  ]
}
```

### 8.1.3. FeatureCollection BBox

The `bbox` (bounding box) is an OPTIONAL field in a FeatureCollection that defines the spatial extent of all features within the collection. Specifying a `bbox` helps GIS systems and APIs quickly ascertain the geographic scope of the data, improving rendering times and spatial queries. The `bbox` is represented as an array of four numbers: [minimum longitude, minimum latitude, maximum longitude, maximum latitude].

Example of the `bbox` field in a `FeatureCollection`:

```json
{
  "type": "FeatureCollection",
  "bbox": [-123.366, 48.428, -123.365, 48.429]
}
```

### 8.1.4. FeatureCollection Attributions

`attributions` are an OPTIONAL but important part of a `FeatureCollection`, providing metadata about the sources of the data. This includes the data provider, the specific terms of use, licensing information, and any other relevant details that must be acknowledged when the data is displayed or utilized. Proper attribution ensures transparency and compliance with data usage policies, and fosters trust and collaboration within the user community.

Example:

```json
{
  "type": "S2FeatureCollection",
  "features": [],
  "faces": [],
  "attributions": {
    "Open S2": "https://opens2.com/legal/data"
  }
}
```

### 8.2. S2FeatureCollection

An `S2FeatureCollection` is a specialized form of `FeatureCollection` designed to handle geographic data encoded using S2 Geometry. This system facilitates efficient spatial indexing and querying by dividing the Earth into a hierarchy of cell regions represented on a spherical surface. The `S2FeatureCollection` is particularly useful in applications requiring high-performance spatial operations, such as large-scale geographic databases and real-time location-based services.

Example of an S2FeatureCollection:

```json
{
  "type": "S2FeatureCollection",
  "features": [
    {
      "type": "S2Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [0.1, 0.5]
      },
      "face": 2,
      "properties": {
        "name": "Example S2 Feature"
      }
    }
  ],
  "faces": [2],
  "bbox": [0.1, 0.5, 0.1, 0.5],
  "attributions": {
    "Open S2": "https://opens2.com/legal/data"
  }
}
```

### 8.2.1. S2FeatureCollection Type

The `type` property of a `S2FeatureCollection` is a fixed string identifier indicating the collection's format. For a standard `S2FeatureCollection`, the `type` MUST be set to `"S2FeatureCollection"`. This property is REQUIRED and specifies the nature of the JSON object as a container for multiple geographical features, ensuring compatibility with GeoJSON standards.

Example of the `type` property in a `S2FeatureCollection`:

```json
{
  "type": "S2FeatureCollection"
}
```

### 8.2.2. S2FeatureCollection Features

The `features` array is a REQUIRED part of a `S2FeatureCollection` and consists of individual Feature objects. Each Feature within the array MUST comply with the S2 Geometry format specifications, containing at least a face, type, properties, and geometry field. This structure not only ensures compliance with broader geospatial data standards but also facilitates integration with various GIS tools and libraries.

Example of the `features` array in a `S2FeatureCollection`:

```json
{
  "type": "S2FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "geometry": {
        "type": "Point",
        "coordinates": [-123.3656, 48.4284]
      },
      "properties": {
        "name": "Example Point"
      }
    }
  ]
}
```

### 8.2.3. S2FeatureCollection BBox

The `bbox` (bounding box) is an OPTIONAL field in an `S2FeatureCollection` that defines the spatial extent of all features within the collection. Similar to a traditional `FeatureCollection`, the `bbox` in an `S2FeatureCollection` provides a rough geographical boundary using longitude and latitude coordinates, despite the collection’s use of S2 Geometry. This approach ensures compatibility with standard GIS systems that expect coordinates in longitude and latitude format.

Example of the `bbox` field in an `S2FeatureCollection`:

```json
{
  "type": "S2FeatureCollection",
  "bbox": [-123.366, 48.428, -123.365, 48.429]
}
```

### 8.2.4. S2FeatureCollection Attributions

`attributions` are an OPTIONAL but important part of a `S2FeatureCollection`, providing metadata about the sources of the data. This includes the data provider, the specific terms of use, licensing information, and any other relevant details that must be acknowledged when the data is displayed or utilized. Proper attribution ensures transparency and compliance with data usage policies, and fosters trust and collaboration within the user community.

Example:

```json
{
  "type": "S2FeatureCollection",
  "features": [],
  "faces": [],
  "attributions": {
    "Open S2": "https://opens2.com/legal/data"
  }
}
```

### 8.2.5. S2FeatureCollection Faces

The `faces` field in an `S2FeatureCollection` is a REQUIREED attribute that provides a quick reference to the specific faces of the S2 cube where the features are located. This field is particularly useful for applications utilizing S2 Geometry, as it directly ties into the system’s method of dividing the sphere into a set of six `faces`, which are then further subdivided into cells.

Example of the `faces` field in an `S2FeatureCollection`:

```json
{
  "type": "S2FeatureCollection",
  "faces": [0, 2, 5]
}
```

## 9. Features

### 9.1. Feature

A `Feature` in this context refers to an individual geographic data point or shape, conforming to the GeoJSON standard. Each `Feature` represents a discrete entity within a `FeatureCollection` and is structured to include a unique identifier, a set of properties, and a geometric definition. This format ensures that each `Feature` can encapsulate all necessary data for various GIS applications, ranging from simple mapping to complex spatial analysis.

A Feature is defined by several key components that describe its physical attributes and metadata:

- **type**: This property is always set to "Feature" for individual geographic entities.
- **id**: A unique identifier for the feature. This can be any number or string that uniquely identifies the feature within its dataset.
- **properties**: A collection of key-value pairs that store descriptive or attribute data relevant to the feature.
- **geometry**: The geometric data that defines the physical shape and location of the feature on the map.

### 9.1.1. Feature ID

The `Feature` `id` is an OPTIONAL attribute, serving as its unique identifier within a `FeatureCollection` or any geospatial dataset. This identifier allows for the distinct referencing, querying, and manipulation of individual features, supporting robust data management and operations in GIS applications.

The `id` MUST be an unsigned integer (greater than or equal to zero).

Example:

```json
{
  "type": "Feature",
  "id": 101,
  "properties": {
    "name": "Statue of Liberty",
    "category": "monument"
  },
  "geometry": {
    "type": "Point",
    "coordinates": [-74.0445, 40.6892]
  }
}
```

### 9.1.2. Feature Properties

The `properties` field in a `Feature` object is a flexible and essential part of GeoJSON, designed to store additional metadata and descriptive information about the geographic entity represented by the feature. This field is structured as a key-value pair dictionary where each key represents a property name, and the value provides details about that property.

The `properties` field is REQUIRED for all `Feature` objects and follows the guidelines outlined in `6.` of this spec.

Example:

```json
{
  "type": "Feature",
  "id": 202,
  "properties": {
    "name": "Golden Gate Bridge",
    "category": "bridge",
    "opened": "1937-05-27",
    "length": "2737m",
    "traffic": "112000",
    "status": "active"
  },
  "geometry": {
    "type": "LineString",
    "coordinates": [
      [-122.4783, 37.8199],
      [-122.4745, 37.8069]
    ]
  }
}
```

### 9.1.3. Feature Geometry

The `geometry` field within a `Feature` object is a core component that defines the spatial characteristics of the feature. This field details the shape and location of the feature on the Earth’s surface, adhering to the GeoJSON format specifications for geometric data types.

For a comprehensive understanding of the different geometric types and their structures, refer to section `10.`.

### 9.2. S2Feature

An `S2Feature` is a specialized type of `Feature` used specifically for geographic data encoded with S2 Geometry. This feature type is designed to efficiently handle data on a spherical surface by utilizing the unique properties of S2 cells and their hierarchical organization.

### 9.2.1 S2Feature ID

The `S2Feature` `id` is an OPTIONAL attribute, serving as its unique identifier within a `S2FeatureCollection` or any geospatial dataset. This identifier allows for the distinct referencing, querying, and manipulation of individual features, supporting robust data management and operations in GIS applications.

The `id` MUST be an unsigned integer (greater than or equal to zero).

Example:

```json
{
  "type": "S2Feature",
  "id": 1337,
  "properties": {
    "name": "Special Region",
    "importance": "High"
  },
  "geometry": {
    "type": "Polygon",
    "coordinates": [[...]]
  },
  "face": 3
}
```

### 9.2.2. S2Feature Properties

The `properties` field in a `S2Feature` object is a flexible and essential part of GeoJSON, designed to store additional metadata and descriptive information about the geographic entity represented by the feature. This field is structured as a key-value pair dictionary where each key represents a property name, and the value provides details about that property.

The `properties` field is REQUIRED for all `S2Feature` objects and follows the guidelines outlined in `6.` of this spec.

Example:

```json
{
  "type": "S2Feature",
  "id": 1337,
  "properties": {
    "name": "Observation Point",
    "visibility": "Clear",
    "accessibility": "Public"
  },
  "geometry": {
    "type": "Point",
    "coordinates": [0.123, -0.456]
  },
  "face": 1
}
```

### 9.2.3. S2Feature Geometry

The `geometry` field within a `S2Feature` object is a core component that defines the spatial characteristics of the feature. This field details the shape and location of the feature on the Earth’s surface, adhering to the GeoJSON format specifications for geometric data types.

`S2Feature` `geometry` differs from `Feature` `geometry` in that it does not describe longitudes and latitudes, but rather the cube's face `s` and `t` coordinates.

For a comprehensive understanding of the different geometric types and their structures, refer to section `10.`.

### 9.2.4. S2Feature Face

The face field is unique to `S2Feature` and is critical for identifying the specific cube face on which the feature’s geometry is projected in the S2 Geometry system. Each face represents a division of the sphere into six parts, which helps in efficiently encoding and querying spatial data. This is more clearly described in section `4.`

> NOTE:
> You may ask yourself: "Why not place the `Face` property inside the geometry object?" However, this is not a constructive solution, since we want to operate on the vector geometry in the same way regardless of the projection type, and so we want to maintain the same geometry interface for all projections.

## 10. Geometry

The `geometry` section within a geospatial data specification delineates the various geometric shapes that can represent features on a map or in a spatial database. `geometry` is foundational to geographical information systems (GIS), as it provides the essential data that describes the physical location and shape of features in the geographic space. Each `geometry` object typically includes several attributes that define its form and function:

- `type`: The type of geometry being described.
- `coordinates`: The coordinates of the geometry.
- `properties`: Additional properties associated with the geometry.
- `bbox`: The bounding box of the geometry.
- `mValues`: M-Values associated with the `coordinates`.

### 10.1. Geometry Type

The `type` field within a `geometry` object defines the type of geometry being described. This field is REQUIRED for all `geometry` objects. The basic types are as follows:

- `Point`: Represents a single point in geographic coordinates.
- `Point3D`: Represents a 3D point in geographic coordinates.
- `MultiPoint`: A collection of points.
- `MultiPoint3D`: A collection of 3D points.
- `LineString`: A series of connected line segments forming a simple polyline.
- `LineString3D`: A series of connected line segments forming a 3D polyline.
- `MultiLineString`: A collection of LineString elements.
- `MultiLineString3D`: A collection of 3D LineString elements.
- `Polygon`: A polygonal area with a boundary and, optionally, holes.
- `Polygon3D`: A 3D polygon with a boundary and, optionally, holes.
- `MultiPolygon`: A collection of Polygon elements.
- `MultiPolygon3D`: A collection of 3D Polygon elements.

Example:

```json
{
  "type": "Point",
  "coordinates": [0.123, -0.456]
}
```

Pseudo-Code of the above type looks like:

```spec
enum GeometryType {
    Point,
    MultiPoint,
    LineString,
    MultiLineString,
    Polygon,
    MultiPolygon,
    Point3D,
    MultiPoint3D,
    LineString3D,
    MultiLineString3D,
    Polygon3D,
    MultiPolygon3D
}
```

### 10.2. Geometry BBox

The `bbox` field within a `geometry` object defines the bounding box of the geometry. This field is OPTIONAL for all `geometry` objects. the `bbox` comes in two forms: 2D and 3D.

For the `Feature` type, the first two numbers MUST be the minimum longitude and latitude, respectively, and the second two numbers MUST be the maximum longitude and latitude, respectively.

For the `S2Feature` type, the first two numbers MUST be the minimum `s` and `t` coordinates, respectively, and the second two numbers MUST be the maximum `s` and `t` coordinates, respectively.

If 3D is used, the last two numbers MUST be the minimum and maximum altitude, respectively. Altitude values are RECOMMENDED to be measured in meters above the surface of the sphere.

#### 10.2.1. Geometry BBox 2D

A 2D bounding box in a GeoJSON context is defined by an array of four numbers.

Example:

```json
{
  "type": "Point",
  "coordinates": [0.123, -0.456],
  "bbox": [-180, -90, 180, 90]
}
```

#### 10.2.2. Geometry BBox 3D

A 3D bounding box in a GeoJSON context is defined by an array of six numbers.

Example:

```json
{
  "type": "Point",
  "coordinates": [0.123, -0.456],
  "bbox": [-180, -90, 180, 90, 0, 100.2]
}
```

### 10.3. Geometry Coordinates

The `coordinates` field within a `geometry` object defines the coordinates of the geometry. This field is REQUIRED for all `geometry` objects.

If type is a `Feature`, then you MUST use `longitude`, `latitude`, and `altitude`. If type is a `S2Feature`, then you MUST use `s`, `t`, and `altitude`.

Altitude values are RECOMMENDED to be measured in meters above the surface of the sphere.

#### 10.3.1. Point

The `Point` geometry type represents a single location in space defined by a pair of coordinates. In a GeoJSON context, a `Point` is typically represented by a longitude and latitude coordinate pair. In systems utilizing S2 Geometry, coordinates are often expressed in terms of the S2 cell ID, or using the (s, t) coordinate system which maps the sphere into a planar space.

The `Point` geometry is used to represent a specific location on the earth, such as a landmark, a city, or any other significant point of interest. It is the simplest form of geometry, without any area or volume.

It is REQUIRED to use a single array of exactly two numbers.

Example in GeoJSON:

```json
{
  "type": "Point",
  "coordinates": [-122.4233, 37.8264]  // Longitude, Latitude
}
```

Example in S2 Geometry:

```json
{
  "type": "Point",
  "coordinates": [0.345, 0.567]  // s, t coordinates
}
```

#### 10.3.2. Point3D

The `Point3D` geometry type extends the simple Point geometry by adding a third dimension, typically representing altitude or depth. This addition allows for a more comprehensive representation of a location in three-dimensional space. In a GeoJSON context, a `Point3D` is represented by a longitude, latitude, and altitude (or elevation) coordinate triplet. In systems utilizing S2 Geometry, the coordinates may include the (s, t) values from the S2 cell ID system, with an additional altitude value.

The `Point3D` geometry is utilized to capture precise locations that require depth information, such as airborne or underwater locations, and is crucial for applications involving three-dimensional modeling and analysis.

It is REQUIRED to use a single array of exactly three numbers to represent this geometry type.

Example in GeoJSON:

```json
{
  "type": "Point",
  "coordinates": [-122.4233, 37.8264, 30.5]  // Longitude, Latitude, Altitude in meters
}
```

Example in S2 Geometry:

```json
{
  "type": "Point",
  "coordinates": [0.345, 0.567, 15.0]  // s, t coordinates, Altitude in meters
}
```

#### 10.3.3. MultiPoint

The `MultiPoint` geometry type represents a collection of points, each defined by a pair of coordinates. In a GeoJSON context, a `MultiPoint` is typically represented by an array of longitude and latitude coordinate pairs. In systems utilizing S2 Geometry, coordinates are often expressed using the (s, t) coordinate system, which maps the sphere into a planar space.

The `MultiPoint` geometry is used to represent multiple specific locations on the Earth, such as a series of landmarks, monitoring stations, or other significant points of interest. It simplifies the handling of groups of points by storing them as a single geometry object, which can be especially useful for operations that involve multiple discrete but related locations.

It is REQUIRED to use an array of arrays, where each sub-array consists of exactly two numbers (representing a point).

Example in GeoJSON:

```json
{
  "type": "MultiPoint",
  "coordinates": [
    [-122.4233, 37.8264],  // Longitude, Latitude of the first point
    [-122.4232, 37.8265],  // Longitude, Latitude of the second point
    [-122.4231, 37.8266]   // Longitude, Latitude of the third point
  ]
}
```

Example in S2 Geometry:

```json
{
  "type": "MultiPoint",
  "coordinates": [
    [0.345, 0.567],  // s, t coordinates of the first point
    [0.346, 0.568],  // s, t coordinates of the second point
    [0.347, 0.569]   // s, t coordinates of the third point
  ]
}
```

#### 10.3.4. MultiPoint3D

The `MultiPoint3D` geometry type extends the `MultiPoint` by incorporating a third dimension, typically representing altitude or depth, for each point. This three-dimensional approach allows for a detailed representation of spatial data that includes elevation, enhancing the utility of the data for various applications that require vertical positioning such as environmental studies, 3D mapping, and urban planning.

In a GeoJSON context, a `MultiPoint3D` is represented by an array of longitude, latitude, and altitude coordinate triplets. In systems utilizing S2 Geometry, the coordinates include the (s, t) values from the S2 cell ID system, supplemented by altitude values.

It is REQUIRED to use an array of arrays, where each sub-array consists of exactly three numbers. These numbers represent a point in three-dimensional space (longitude, latitude, altitude), or (s, t, altitude).

Example in GeoJSON:

```json
{
  "type": "MultiPoint",
  "coordinates": [
    [-122.4233, 37.8264, 10.0],  // Longitude, Latitude, Altitude of the first point
    [-122.4232, 37.8265, 20.0],  // Longitude, Latitude, Altitude of the second point
    [-122.4231, 37.8266, 30.0]   // Longitude, Latitude, Altitude of the third point
  ]
}
```

Example in S2 Geometry:

```json
{
  "type": "MultiPoint",
  "coordinates": [
    [0.345, 0.567, 5.0],  // s, t coordinates, Altitude of the first point
    [0.346, 0.568, 10.0],  // s, t coordinates, Altitude of the second point
    [0.347, 0.569, 15.0]   // s, t coordinates, Altitude of the third point
  ]
}
```

#### 10.3.5. LineString

The `LineString` geometry type represents a series of connected points that form a continuous line. It is commonly used to map out roads, paths, or any linear geographical feature on a map. In a GeoJSON context, a `LineString` is depicted by an array of longitude and latitude coordinate pairs, which define the sequence of points that make up the line. In S2 Geometry systems, coordinates are expressed using the (s, t) coordinate system, mapping points on a spherical surface to a two-dimensional plane relative to the `face` of the S2 cell.

A `LineString` is REQUIRED to contain an array of at least two coordinate pairs, as a single coordinate pair does not suffice to define a line.

Example in GeoJSON:

```json
{
  "type": "LineString",
  "coordinates": [
    [-122.4233, 37.8264],  // Longitude, Latitude of the first point
    [-122.4232, 37.8265],  // Longitude, Latitude of the second point
    [-122.4231, 37.8266]   // Longitude, Latitude of the third point
  ]
}
```

Example in S2 Geometry:

```json
{
  "type": "LineString",
  "coordinates": [
    [0.345, 0.567],  // s, t coordinates of the first point
    [0.346, 0.568],  // s, t coordinates of the second point
    [0.347, 0.569]   // s, t coordinates of the third point
  ]
}
```

#### 10.3.6. LineString3D

The `LineString3D` geometry type extends the traditional LineString by adding a third dimension, typically altitude or depth. This enhancement allows the `LineString3D` to represent not only the path along the surface of the earth but also variations in elevation along that path, providing a more detailed and accurate depiction of features such as mountain trails, overhead cables, or underwater pipelines.

In a GeoJSON context, a `LineString3D` is defined by an array of longitude, latitude, and altitude coordinate triplets. For systems utilizing S2 Geometry, the coordinates would include the (s, t) values plus an altitude component, providing a comprehensive three-dimensional mapping of each point in the line.

A `LineString3D` is REQUIRED to contain an array of at least two coordinate triplets, as at least two points are necessary to define a three-dimensional line.

Example in GeoJSON:

```json
{
  "type": "LineString",
  "coordinates": [
    [-122.4233, 37.8264, 5.0],  // Longitude, Latitude, Altitude of the first point
    [-122.4232, 37.8265, 10.0],  // Longitude, Latitude, Altitude of the second point
    [-122.4231, 37.8266, 15.0]   // Longitude, Latitude, Altitude of the third point
  ]
}
```

Example in S2 Geometry:

```json
{
  "type": "LineString",
  "coordinates": [
    [0.345, 0.567, 2.0],  // s, t coordinates, Altitude of the first point
    [0.346, 0.568, 3.0],  // s, t coordinates, Altitude of the second point
    [0.347, 0.569, 4.0]   // s, t coordinates, Altitude of the third point
  ]
}
```

#### 10.3.7. MultiLineString

The `MultiLineString` geometry type represents a collection of `LineString` geometries, each defined as a sequence of points. This type is used when you need to represent multiple lines as a single geometric entity, such as multiple streets, streams, or any other linear features that are related but not necessarily connected.

In a GeoJSON context, a `MultiLineString` is depicted by an array of arrays, where each sub-array is a LineString consisting of longitude and latitude coordinate pairs. In systems utilizing S2 Geometry, `MultiLineString` coordinates are expressed using the (s, t) system, effectively mapping these points on the sphere to a planar surface.

A `MultiLineString` is REQUIRED to contain at least two LineString arrays, each with at least two coordinate pairs, to maintain its definition as a multi-line entity.

Example in GeoJSON:

```json
{
  "type": "MultiLineString",
  "coordinates": [
    [ // First LineString
      [-122.4233, 37.8264],
      [-122.4232, 37.8265],
      [-122.4231, 37.8266]
    ],
    [ // Second LineString
      [-122.4243, 37.8274],
      [-122.4242, 37.8275],
      [-122.4241, 37.8276]
    ]
  ]
}
```

Example in S2 Geometry:

```json
{
  "type": "MultiLineString",
  "coordinates": [
    [ // First LineString
      [0.345, 0.567],
      [0.346, 0.568],
      [0.347, 0.569]
    ],
    [ // Second LineString
      [0.348, 0.570],
      [0.349, 0.571],
      [0.350, 0.572]
    ]
  ]
}
```

#### 10.3.8. MultiLineString3D

The `MultiLineString3D` geometry type extends the capabilities of the `MultiLineString` by incorporating a third dimension, typically altitude or depth, for each line in the collection. This addition enhances the description of linear features by providing information not only about their location on the earth’s surface but also their vertical position, which is crucial for applications that require detailed environmental modeling, three-dimensional urban planning, or complex engineering projects.

In a GeoJSON context, a `MultiLineString3D` is defined by an array of LineString arrays, where each LineString is composed of longitude, latitude, and altitude coordinate triplets. In systems utilizing S2 Geometry, coordinates would include the (s, t) values along with an altitude component, thus giving a complete three-dimensional mapping of each line.

A `MultiLineString3D` is REQUIRED to contain at least two LineString arrays, each with at least two coordinate triplets, ensuring that it retains its multi-line characteristic in three-dimensional space.

Example in GeoJSON:

```json
{
  "type": "MultiLineString",
  "coordinates": [
    [ // First LineString
      [-122.4233, 37.8264, 10.0],
      [-122.4232, 37.8265, 15.0],
      [-122.4231, 37.8266, 20.0]
    ],
    [ // Second LineString
      [-122.4243, 37.8274, 10.0],
      [-122.4242, 37.8275, 15.0],
      [-122.4241, 37.8276, 20.0]
    ]
  ]
}
```

Example in S2 Geometry:

```json
{
  "type": "MultiLineString",
  "coordinates": [
    [ // First LineString
      [0.345, 0.567, 2.0],
      [0.346, 0.568, 3.0],
      [0.347, 0.569, 4.0]
    ],
    [ // Second LineString
      [0.348, 0.570, 2.0],
      [0.349, 0.571, 3.0],
      [0.350, 0.572, 4.0]
    ]
  ]
}
```

#### 10.3.9. Polygon

The `Polygon` geometry type represents a two-dimensional shape consisting of a series of linear rings that define the boundaries of the shape. In geographic data representations like GeoJSON, a `Polygon` is typically used to define areas such as city boundaries, land uses, lake outlines, or other regions that require enclosure.

In a GeoJSON context, a `Polygon` is depicted by an array of arrays of coordinate pairs, where the first array represents the outer boundary of the polygon, and any subsequent arrays represent holes or internal boundaries within the polygon. For systems using S2 Geometry, the `Polygon` coordinates are expressed using the (s, t) system, effectively mapping these boundaries on the spherical surface to a planar projection.

A `Polygon` is REQUIRED to contain at least one array (representing the outer boundary), and each boundary array MUST be closed, meaning the first and last coordinate pair in the array MUST be identical. Additionally, the arrays MUST define a linear ring that does not self-intersect. Outer boundary arrays MUST be counter-clockwise ordered. Hole arrays MUST be clockwise ordered. The length of each polygon's boundary array MUST be greater than 3.

Example in GeoJSON:

```json
{
  "type": "Polygon",
  "coordinates": [
    [ // Outer boundary
      [-122.4233, 37.8264],
      [-122.4232, 37.8265],
      [-122.4231, 37.8266],
      [-122.4233, 37.8264] // Closing the loop
    ],
    [ // Inner hole
      [-122.42325, 37.82645],
      [-122.42315, 37.82655],
      [-122.42305, 37.82665],
      [-122.42325, 37.82645] // Closing the loop
    ]
  ]
}
```

Example in S2 Geometry:

```json
{
  "type": "Polygon",
  "coordinates": [
    [ // Outer boundary
      [0.345, 0.567],
      [0.346, 0.568],
      [0.347, 0.569],
      [0.345, 0.567] // Closing the loop
    ]
  ]
}
```

#### 10.3.10. Polygon3D

The `Polygon3D` geometry type extends the basic `Polygon` by adding a third dimension, typically representing altitude or depth. This enhancement allows the `Polygon3D` to provide a more comprehensive representation of spatial areas, incorporating elevation data to define three-dimensional shapes such as building footprints, terrain models, or volumetric spaces.

In a GeoJSON context, a `Polygon3D` is depicted by an array of arrays of coordinate triplets, where each triplet includes longitude, latitude, and altitude. In systems utilizing S2 Geometry, the `Polygon3D` coordinates are expressed using the (s, t) system, effectively mapping these boundaries on the spherical surface to a planar projection.

A `Polygon3D` is REQUIRED to contain at least one array (representing the outer boundary), and each boundary array MUST be closed, meaning the first and last coordinate triplet in the array must be identical. Additionally, the arrays MUST define a linear ring that does not self-intersect. Outer boundary arrays MUST be counter-clockwise ordered. Hole arrays MUST be clockwise ordered. The length of each polygon's boundary array MUST be greater than 3.

Example in GeoJSON:

```json
{
  "type": "Polygon",
  "coordinates": [
    [ // Outer boundary with altitude
      [-122.4233, 37.8264, 5.0],
      [-122.4232, 37.8265, 5.0],
      [-122.4231, 37.8266, 5.0],
      [-122.4233, 37.8264, 5.0] // Closing the loop, counter-clockwise order
    ],
    [ // Inner hole with altitude
      [-122.42325, 37.82645, 10.0],
      [-122.42315, 37.82655, 10.0],
      [-122.42305, 37.82665, 10.0],
      [-122.42325, 37.82645, 10.0] // Closing the loop, clockwise order
    ]
  ]
}
```

Example in S2 Geometry:

```json
{
  "type": "Polygon",
  "coordinates": [
    [ // Outer boundary with altitude
      [0.345, 0.567, 2.0],
      [0.346, 0.568, 2.0],
      [0.347, 0.569, 2.0],
      [0.345, 0.567, 2.0] // Closing the loop, counter-clockwise order
    ]
  ]
}
```

#### 10.3.11. MultiPolygon

The `MultiPolygon` geometry type represents a collection of `Polygon` geometries, allowing for the definition of multiple separate or overlapping polygonal areas within a single geometric entity. This type is particularly useful for representing complex spatial structures, such as multiple islands in an archipelago, distinct water bodies within a landscape, or various parcels of land within a municipality.

In a GeoJSON context, a `MultiPolygon` is depicted by an array of `Polygon` arrays, where each sub-array represents a `Polygon` with its own set of boundary arrays. Each `Polygon` follows the standard rules, including the closure of boundaries and the specific winding order: outer boundaries MUST be counter-clockwise, and holes MUST be clockwise.

A `MultiPolygon` is REQUIRED to contain at least one `Polygon`, and each `Polygon` in the array MUST adhere to the standard requirements of being closed and non-self-intersecting. Each boundary in these polygons MUST also follow the prescribed winding order to accurately represent the area and any internal exclusions. The length of each polygon's boundary array MUST be greater than 3.

Example in GeoJSON:

```json
{
  "type": "MultiPolygon",
  "coordinates": [
    [ // First Polygon
      [ // Outer boundary
        [-122.4233, 37.8264],
        [-122.4232, 37.8265],
        [-122.4231, 37.8266],
        [-122.4233, 37.8264] // Closing the loop, counter-clockwise order
      ]
    ],
    [ // Second Polygon
      [ // Outer boundary
        [-122.4223, 37.8274],
        [-122.4222, 37.8275],
        [-122.4221, 37.8276],
        [-122.4223, 37.8274] // Closing the loop, counter-clockwise order
      ],
      [ // Inner hole
        [-122.42225, 37.82745],
        [-122.42215, 37.82755],
        [-122.42205, 37.82765],
        [-122.42225, 37.82745] // Closing the loop, clockwise order
      ]
    ]
  ]
}
```

Example in S2 Geometry:

```json
{
  "type": "MultiPolygon",
  "coordinates": [
    [ // First Polygon
      [ // Outer boundary
        [0.345, 0.567],
        [0.346, 0.568],
        [0.347, 0.569],
        [0.345, 0.567] // Closing the loop, counter-clockwise order
      ]
    ],
    [ // Second Polygon
      [ // Outer boundary
        [0.348, 0.570],
        [0.349, 0.571],
        [0.350, 0.572],
        [0.348, 0.570] // Closing the loop, counter-clockwise order
      ],
      [ // Inner hole
        [0.349, 0.571],
        [0.3495, 0.5715],
        [0.349, 0.572],
        [0.349, 0.571] // Closing the loop, clockwise order
      ]
    ]
  ]
}
```

#### 10.3.12. MultiPolygon3D

The `MultiPolygon3D` geometry type extends the `MultiPolygon` by incorporating a third dimension, typically altitude or depth, into each polygon. This type allows for the definition of complex, volumetric spatial structures, making it particularly useful for applications that require detailed modeling of geographic features with varying elevations, such as geological formations, layered environmental regions, or multi-level urban developments.

In a GeoJSON context, a `MultiPolygon3D` is represented by an array of Polygon arrays, where each sub-array is a Polygon with its own set of boundary arrays of coordinate triplets, including longitude, latitude, and altitude. Each Polygon adheres to the same structural rules as in 2D, including the closure of boundaries and the specific winding order: outer boundaries MUST be counter-clockwise, and holes MUST be clockwise.

A `MultiPolygon3D` is REQUIRED to contain at least one Polygon, and each Polygon in the array MUST adhere to the standards of being closed and non-self-intersecting. Each boundary in these polygons MUST also follow the prescribed winding order and MUST have at least four coordinate triplets (including the repeated closing coordinate) to accurately represent the three-dimensional area and any internal exclusions.

Example in GeoJSON:

```json
{
  "type": "MultiPolygon",
  "coordinates": [
    [ // First Polygon
      [ // Outer boundary
        [-122.4233, 37.8264, 5.0],
        [-122.4232, 37.8265, 5.0],
        [-122.4231, 37.8266, 5.0],
        [-122.4233, 37.8264, 5.0] // Closing the loop, counter-clockwise order
      ]
    ],
    [ // Second Polygon
      [ // Outer boundary
        [-122.4223, 37.8274, 10.0],
        [-122.4222, 37.8275, 10.0],
        [-122.4221, 37.8276, 10.0],
        [-122.4223, 37.8274, 10.0] // Closing the loop, counter-clockwise order
      ],
      [ // Inner hole
        [-122.42225, 37.82745, 15.0],
        [-122.42215, 37.82755, 15.0],
        [-122.42205, 37.82765, 15.0],
        [-122.42225, 37.82745, 15.0] // Closing the loop, clockwise order
      ]
    ]
  ]
}
```

Example in S2 Geometry:

```json
{
  "type": "MultiPolygon",
  "coordinates": [
    [ // First Polygon
      [ // Outer boundary
        [0.345, 0.567, 2.0],
        [0.346, 0.568, 2.0],
        [0.347, 0.569, 2.0],
        [0.345, 0.567, 2.0] // Closing the loop, counter-clockwise order
      ]
    ],
    [ // Second Polygon
      [ // Outer boundary
        [0.348, 0.570, 3.0],
        [0.349, 0.571, 3.0],
        [0.350, 0.572, 3.0],
        [0.348, 0.570, 3.0] // Closing the loop, counter-clockwise order
      ],
      [ // Inner hole
        [0.349, 0.571, 4.0],
        [0.3495, 0.5715, 4.0],
        [0.349, 0.572, 4.0],
        [0.349, 0.571, 4.0] // Closing the loop, clockwise order
      ]
    ]
  ]
}
```

### 10.4. Geometry MValues

A geometries M-Values are OPTIONAL. The M-Values size and length always match the geometries coordinates length and size. For example, if the Geometry type is a `LineString` the M-Values is also a `LineString` in shape, and it's length matches the coordinates length.

all M-Values MUST follow the same structure with the same key-value pairs.

M-Values are REQUIRED to behave identical to `properties` defined in section `6.` and follows the same guidelines.

Example:

```json
{
  "type": "MultiLineString",
  "coordinates": [
    [
      [100.0, 0.0],
      [101.0, 1.0]
    ],
    [
      [102.0, 2.0],
      [103.0, 3.0]
    ]
  ],
  "mValues": [
    [
      { "foo": "bar" },
      { "foo": "baz" }
    ],
    [
      { "foo": "qux" },
      { "foo": "quux" }
    ]
  ]
}
```
