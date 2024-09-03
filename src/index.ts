import type { BBox, Geometry, VectorGeometry } from './geometry.spec';
import type { MValue, Properties } from './values.spec';

export * from './s2';
export * from './wm';
export * from './bbox';
export * from './id';
export * from './simplify';
export * from './tile';
export * from './util';

export * from './geometry.spec';
export * from './values.spec';

// import * as schema from './s2json.schema.json';
// export { schema };

// NOTE: S2 -> S2Geometry
// NOTE: WG -> WGS84

/** Whether the projection is S2 or WM */
export type Projection = 'WM' | 'S2';

/// S2 specific type

/** cube-face on the S2 sphere */
export type Face = 0 | 1 | 2 | 3 | 4 | 5;

/// FeatureCollections

/** Types will either be an S2 or WG FeatureCollection */
export type FeatureCollectionType = 'FeatureCollection' | 'S2FeatureCollection';
/** Either an S2 or WG FeatureCollection */
export interface BaseFeatureCollection<T = FeatureCollectionType, F = Features> {
  type: T;
  features: F[];
  attributions?: Attributions;
  bbox?: BBox;
}
/** WG FeatureCollection */
export type FeatureCollection = BaseFeatureCollection<'FeatureCollection', Feature | VectorFeature>;
/** S2 FeatureCollection */
export interface S2FeatureCollection
  extends BaseFeatureCollection<'S2FeatureCollection', S2Feature> {
  faces: Face[];
}

/// Features

/** Either an S2 or WG Feature type */
export type FeatureType = 'Feature' | 'VectorFeature' | 'S2Feature';
/** Base component to build either an S2 or WG Feature */
export interface BaseFeature<
  T = FeatureType,
  P extends Properties = Properties,
  G = Geometry<MValue> | VectorGeometry,
> {
  type: T;
  id?: number;
  properties: P;
  geometry: G;
  metadata?: Record<string, unknown>;
}
/** WG Feature */
export type Feature<
  P extends Properties = Properties,
  M extends MValue = MValue,
  G = Geometry<M>,
> = BaseFeature<'Feature', P, G>;
/** WG Vector Feature */
export type VectorFeature<P extends Properties = Properties, G = VectorGeometry> = BaseFeature<
  'VectorFeature',
  P,
  G
>;
/** S2 Feature */
export interface S2Feature<P extends Properties = Properties, G = VectorGeometry>
  extends BaseFeature<'S2Feature', P, G> {
  face: Face;
}

/// Utility types

/**
 * Attribution data is stored in an object.
 * The key is the name of the attribution, and the value is the href link
 * e.g. { "Open S2": "https://opens2.com/legal/data" }
 */
export type Attributions = Record<string, string>;

/** Either an S2 or WG FeatureCollection */
export type FeatureCollections = FeatureCollection | S2FeatureCollection;

/** Either an S2 or WG Feature */
export type Features = Feature | VectorFeature | S2Feature;

/** Any Vector Geometry type */
export type VectorFeatures = VectorFeature | S2Feature;

/** All major S2JSON types */
export type JSONCollection = FeatureCollection | S2FeatureCollection | Features;
