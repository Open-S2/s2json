import type { BBox, Geometry } from './geometry';
import type { MValue, Properties } from './values';

export * from './geometry';
export * from './values';

// NOTE: S2 -> S2Geometry
// NOTE: WG -> WGS84

//! S2 specific type

/** cube-face on the S2 sphere */
export type Face = 0 | 1 | 2 | 3 | 4 | 5;

//! FeatureCollections

/** Types will either be an S2 or WG FeatureCollection */
export type FeatureCollectionType = 'FeatureCollection' | 'S2FeatureCollection';
/** Either an S2 or WG FeatureCollection */
export interface BaseFeatureCollection<T = FeatureCollectionType, F = Feature | S2Feature> {
  type: T;
  features: F[];
  attributions?: Attributions;
  bbox?: BBox;
}
/** WG FeatureCollection */
export interface FeatureCollection extends BaseFeatureCollection<'FeatureCollection', Feature> {}
/** S2 FeatureCollection */
export interface S2FeatureCollection
  extends BaseFeatureCollection<'S2FeatureCollection', S2Feature> {
  faces: Face[];
}

//! Features

/** Either an S2 or WG Feature type */
export type FeatureType = 'Feature' | 'S2Feature';
/** Base component to build either an S2 or WG Feature */
export interface BaseFeature<
  T = FeatureType,
  P extends Properties = Properties,
  M extends MValue = MValue,
> {
  type: T;
  id?: number;
  properties: P;
  geometry: Geometry<M>;
  metadata?: Record<string, unknown>;
}
/** WG Feature */
export interface Feature<P extends Properties = Properties, M extends MValue = MValue>
  extends BaseFeature<'Feature', P, M> {}
/** S2 Feature */
export interface S2Feature<P extends Properties = Properties, M extends MValue = MValue>
  extends BaseFeature<'S2Feature', P, M> {
  face: Face;
}

//! Utility types

/**
 * Attribution data is stored in an object.
 * The key is the name of the attribution, and the value is the href link
 * e.g. { "Open S2": "https://opens2.com/legal/data" }
 */
export type Attributions = Record<string, string>;

/** Either an S2 or WG FeatureCollection */
export type FeatureCollections = FeatureCollection | S2FeatureCollection;

/** Either an S2 or WG Feature */
export type Features = Feature | S2Feature;

/** All major S2JSON types */
export type JSONCollection = FeatureCollection | S2FeatureCollection | Feature | S2Feature;
