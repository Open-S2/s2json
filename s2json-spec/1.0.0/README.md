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

### 3.1. WGS84 (EPSG:4326)

### 3.2. S2 (S2 Geometry)

## 4. S2 Face

In S2 Geometry, the world is represented as a unit sphere and divided into six faces using a cube -like projection. Each face of the cube is identified by an integer index from 0 to 5 inclusive.

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

## 5. Attributions

Attributions provide metadata about the sources of data used in the S2JSON file. They typically include information about the data provider, licensing, and other relevant details that should be displayed when the data is used or visualized.

Attributions are OPTIONAL.

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

5.2. Array Values

Array values are ordered lists of values. Once you use an array, each item must be of the same type. Each item can be either a primitive value or an object whose keys are strings and values are primitive values.

Example:

```json
{
  "arrayValue": [1, 2, 3, 4, 5]
}
```

5.3. Object Values (nested values)

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

Properties are key-value pairs that store additional data about features. The key is a string and the value is any Value type: `primitive`, `array`, or `object`.

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

### 8.1. FeatureCollection

### 8.1.1. FeatureCollection Type

### 8.1.2. FeatureCollection Features

### 8.1.3. FeatureCollection BBox

### 8.1.4. FeatureCollection Attributions

### 8.2. S2FeatureCollection

### 8.2.1. S2FeatureCollection Type

### 8.2.2. S2FeatureCollection Features

### 8.2.3. S2FeatureCollection BBox

### 8.2.4. S2FeatureCollection Attributions

### 8.2.5. S2FeatureCollection Faces

## 9. Features

### 9.1. Feature

### 9.1.1. Feature ID

### 9.1.2. Feature Properties

### 9.1.3. Feature Geometry

### 9.2. S2Feature

### 9.2.1 S2Feature ID

### 9.2.2. S2Feature Properties

### 9.2.3. S2Feature Geometry

### 9.2.4. S2Feature Face

You may ask yourself: "Why not place the `Face` property inside the geometry object?" However, this is not a constructive solution, since we want to operate on the vector geometry in the same way regardless of the projection type, and so we want to maintain the same geometry interface for all projections.

## 10. Geometry

### 10.1. Geometry Types

### 10.2. Geometry BBox

#### 10.2.1. Geometry BBox 2D

#### 10.2.2. Geometry BBox 3D

### 10.3. Geometry Coordinates

#### 10.3.1. Point

#### 10.3.2. Point3D

#### 10.3.3. MultiPoint

#### 10.3.4. MultiPoint3D

#### 10.3.5. LineString

#### 10.3.6. LineString3D

#### 10.3.7. MultiLineString

#### 10.3.8. MultiLineString3D

#### 10.3.9. Polygon

#### 10.3.10. Polygon3D

#### 10.3.11. MultiPolygon

#### 10.3.12. MultiPolygon3D

### 10.4. Geometry MValues

A geometries M-Values are OPTIONAL. The M-Values size and length always match the geometries coordinates length and size. For example, if the Geometry type is a `LineString` the M-Values is also a `LineString` in shape, and it's length matches the coordinates length.

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

## 11. Examples
