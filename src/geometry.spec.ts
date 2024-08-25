import type { Face } from '.';
import type {
  LineStringMValues,
  MValue,
  MValues,
  MultiLineStringMValues,
  MultiPolygonMValues,
  PolygonMValues,
} from './values.spec';

/**
 * A BBOX is defined in lon-lat space and helps with zooming motion to
 * see the entire line or polygon
 */
export type BBox = [left: number, bottom: number, right: number, top: number];
/**
 * A BBOX is defined in lon-lat space and helps with zooming motion to
 * see the entire 3D line or polygon
 */
export type BBox3D = [
  left: number,
  bottom: number,
  right: number,
  top: number,
  front: number,
  back: number,
];

/** Either a 2D or 3D bounding box */
export type BBOX = BBox | BBox3D;

/** A Point in S2 Space with a Face */
export type STPoint = [face: Face, s: number, t: number];

/** Definition of a Point. May represent WebMercator Lon-Lat or S2Geometry S-T */
export type Point = [x: number, y: number];
/** Definition of a MultiPoint */
export type MultiPoint = Point[];
/** Definition of a LineString */
export type LineString = Point[];
/** Definition of a MultiLineString */
export type MultiLineString = LineString[];
/** Definition of a Polygon */
export type Polygon = Point[][];
/** Definition of a MultiPolygon */
export type MultiPolygon = Polygon[];
/** Definition of a 3D Point. May represent WebMercator Lon-Lat or S2Geometry S-T with a z-value */
export type Point3D = [x: number, y: number, z: number];
/** Definition of a 3D MultiPoint */
export type MultiPoint3D = Point3D[];
/** Definition of a 3D LineString */
export type LineString3D = Point3D[];
/** Definition of a 3D MultiLineString */
export type MultiLineString3D = LineString3D[];
/** Definition of a 3D Polygon */
export type Polygon3D = Point3D[][];
/** Definition of a 3D MultiPolygon */
export type MultiPolygon3D = Polygon3D[];
/** All possible geometry coordinates */
export type Coordinates =
  | Point
  | MultiPoint
  | LineString
  | MultiLineString
  | Polygon
  | MultiPolygon
  | Point3D
  | MultiPoint3D
  | LineString3D
  | MultiLineString3D
  | Polygon3D
  | MultiPolygon3D;

/** All possible geometry types */
export type GeometryType =
  | 'Point'
  | 'MultiPoint'
  | 'LineString'
  | 'MultiLineString'
  | 'Polygon'
  | 'MultiPolygon'
  | 'Point3D'
  | 'MultiPoint3D'
  | 'LineString3D'
  | 'MultiLineString3D'
  | 'Polygon3D'
  | 'MultiPolygon3D';
/** All possible geometry shapes */
export type Geometry<M extends MValue = MValue> =
  // 2D
  | PointGeometry<M>
  | MultiPointGeometry<M>
  | LineStringGeometry<M>
  | MultiLineStringGeometry<M>
  | PolygonGeometry<M>
  | MultiPolygonGeometry<M>
  // 3D
  | Point3DGeometry<M>
  | MultiPoint3DGeometry<M>
  | LineString3DGeometry<M>
  | MultiLineString3DGeometry<M>
  | Polygon3DGeometry<M>
  | MultiPolygon3DGeometry<M>;

/** BaseGeometry with MValues is the a generic geometry type that includes MValues */
export interface BaseGeometry<T = GeometryType, C = Coordinates, M = MValues, B = BBOX> {
  type: T;
  coordinates: C;
  mValues?: M;
  bbox?: B;
}

/** PointGeometry is a point */
export interface PointGeometry<M extends MValue = MValue>
  extends BaseGeometry<'Point', Point, M, BBox> {}
/** MultiPointGeometry contains multiple points */
export interface MultiPointGeometry<M extends MValue = MValue>
  extends BaseGeometry<'MultiPoint', MultiPoint, LineStringMValues<M>, BBox> {}
/** LineStringGeometry is a line */
export interface LineStringGeometry<M extends MValue = MValue>
  extends BaseGeometry<'LineString', LineString, LineStringMValues<M>, BBox> {}
/** MultiLineStringGeometry contians multiple lines */
export interface MultiLineStringGeometry<M extends MValue = MValue>
  extends BaseGeometry<'MultiLineString', MultiLineString, MultiLineStringMValues<M>, BBox> {}
/** PolygonGeometry is a polygon with potential holes */
export interface PolygonGeometry<M extends MValue = MValue>
  extends BaseGeometry<'Polygon', Polygon, PolygonMValues<M>, BBox> {}
/** MultiPolygonGeometry is a polygon with multiple polygons with their own potential holes */
export interface MultiPolygonGeometry<M extends MValue = MValue>
  extends BaseGeometry<'MultiPolygon', MultiPolygon, MultiPolygonMValues<M>, BBox> {}
/** Point3DGeometry is a 3D point */
export interface Point3DGeometry<M extends MValue = MValue>
  extends BaseGeometry<'Point3D', Point3D, M, BBox3D> {}
/** MultiPoint3DGeometry contains multiple 3D points */
export interface MultiPoint3DGeometry<M extends MValue = MValue>
  extends BaseGeometry<'MultiPoint3D', MultiPoint3D, LineStringMValues<M>, BBox3D> {}
/** LineString3DGeometry is a 3D line */
export interface LineString3DGeometry<M extends MValue = MValue>
  extends BaseGeometry<'LineString3D', LineString3D, LineStringMValues<M>, BBox3D> {}
/** MultiLineString3DGeometry contians multiple 3D lines */
export interface MultiLineString3DGeometry<M extends MValue = MValue>
  extends BaseGeometry<'MultiLineString3D', MultiLineString3D, MultiLineStringMValues<M>, BBox3D> {}
/** Polygon3DGeometry is a 3D polygon with potential holes */
export interface Polygon3DGeometry<M extends MValue = MValue>
  extends BaseGeometry<'Polygon3D', Polygon3D, PolygonMValues<M>, BBox3D> {}
/** MultiPolygon3DGeometry is a 3D polygon with multiple polygons with their own potential holes */
export interface MultiPolygon3DGeometry<M extends MValue = MValue>
  extends BaseGeometry<'MultiPolygon3D', MultiPolygon3D, MultiPolygonMValues<M>, BBox3D> {}
