import type {
  LineStringMValues,
  MValue,
  MValues,
  MultiLineStringMValues,
  MultiPolygonMValues,
  PolygonMValues,
} from './values';

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

/** All possible geometry types */
export type GeometryType =
  | 'Point'
  | 'MultiPoint'
  | 'LineString'
  | 'MultiLineString'
  | 'Polygon'
  | 'MultiPolygon';
/** All possible geometry shapes */
export type Geometry =
  // 2D
  | PointGeometry
  | MultiPointGeometry
  | LineStringGeometry
  | MultiLineStringGeometry
  | PolygonGeometry
  | MultiPolygonGeometry
  // 3D
  | Point3DGeometry
  | MultiPoint3DGeometry
  | LineString3DGeometry
  | MultiLineString3DGeometry
  | Polygon3DGeometry
  | MultiPolygon3DGeometry;

/** BaseGeometry with MValues is the a generic geometry type that includes MValues */
export interface BaseGeometry<T = GeometryType, G = Geometry, M = MValues, B = BBOX> {
  type: T;
  coordinates: G;
  mValues?: M;
  bbox?: B;
}

/** PointGeometry is a point */
export interface PointGeometry extends BaseGeometry<'Point', Point, MValue, BBox> {}
/** MultiPointGeometry contains multiple points */
export interface MultiPointGeometry extends BaseGeometry<'MultiPoint', MultiPoint, MValue, BBox> {}
/** LineStringGeometry is a line */
export interface LineStringGeometry
  extends BaseGeometry<'LineString', LineString, LineStringMValues, BBox> {}
/** MultiLineStringGeometry contians multiple lines */
export interface MultiLineStringGeometry
  extends BaseGeometry<'MultiLineString', MultiLineString, MultiLineStringMValues, BBox> {}
/** PolygonGeometry is a polygon with potential holes */
export interface PolygonGeometry extends BaseGeometry<'Polygon', Polygon, PolygonMValues, BBox> {}
/** MultiPolygonGeometry is a polygon with multiple polygons with their own potential holes */
export interface MultiPolygonGeometry
  extends BaseGeometry<'MultiPolygon', MultiPolygon, MultiPolygonMValues, BBox> {}
/** Point3DGeometry is a 3D point */
export interface Point3DGeometry extends BaseGeometry<'Point', Point3D, MValue, BBox3D> {}
/** MultiPoint3DGeometry contains multiple 3D points */
export interface MultiPoint3DGeometry
  extends BaseGeometry<'MultiPoint', MultiPoint3D, MValue, BBox3D> {}
/** LineString3DGeometry is a 3D line */
export interface LineString3DGeometry
  extends BaseGeometry<'LineString', LineString3D, LineStringMValues, BBox3D> {}
/** MultiLineString3DGeometry contians multiple 3D lines */
export interface MultiLineString3DGeometry
  extends BaseGeometry<'MultiLineString', MultiLineString3D, MultiLineStringMValues, BBox3D> {}
/** Polygon3DGeometry is a 3D polygon with potential holes */
export interface Polygon3DGeometry
  extends BaseGeometry<'Polygon', Polygon3D, PolygonMValues, BBox3D> {}
/** MultiPolygon3DGeometry is a 3D polygon with multiple polygons with their own potential holes */
export interface MultiPolygon3DGeometry
  extends BaseGeometry<'MultiPolygon', MultiPolygon3D, MultiPolygonMValues, BBox3D> {}
