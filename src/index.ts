import type { BBOX, Geometry, VectorGeometry } from './geometry.spec';
import type { MValue, Properties } from './values.spec';

export * from './geometry.spec';
export * from './values.spec';

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
  bbox?: BBOX;
}
/** WG FeatureCollection */
export type FeatureCollection<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
  G = Geometry<D> | VectorGeometry<D>,
> = BaseFeatureCollection<'FeatureCollection', Feature<M, D, P, G> | VectorFeature<M, D, P, G>>;
/** S2 FeatureCollection */
export interface S2FeatureCollection<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
  G = VectorGeometry<D>,
> extends BaseFeatureCollection<'S2FeatureCollection', S2Feature<M, D, P, G>> {
  faces: Face[];
}

/// Features

/** Either an S2 or WG Feature type */
export type FeatureType = 'Feature' | 'VectorFeature' | 'S2Feature';
/** Base component to build either an S2 or WG Feature */
export interface BaseFeature<
  T = FeatureType,
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
  G = Geometry<D> | VectorGeometry<D>,
> {
  type: T;
  id?: number;
  face?: Face;
  properties: P;
  geometry: G;
  metadata?: M;
}
/** WG Feature */
export type Feature<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
  G = Geometry<D>,
> = BaseFeature<'Feature', M, D, P, G>;
/** WG Vector Feature */
export type VectorFeature<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
  G = VectorGeometry<D>,
> = BaseFeature<'VectorFeature', M, D, P, G>;
/** S2 Feature */
export interface S2Feature<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
  G = VectorGeometry<D>,
> extends BaseFeature<'S2Feature', M, D, P, G> {
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
export type FeatureCollections<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
  G = Geometry<D> | VectorGeometry<D>,
> = FeatureCollection<M, D, P, G> | S2FeatureCollection<M, D, P, G>;

/** Either an S2 or WG FeatureCollection where its known it's only Vector Geometry */
export type VectorFeatureCollections<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
  G extends VectorGeometry<D> = VectorGeometry<D>,
> = FeatureCollection<M, D, P, G> | S2FeatureCollection<M, D, P, G>;

/** Either an S2 or WG Feature */
export type Features<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
  G = Geometry<D> | VectorGeometry<D>,
> = Feature<M, D, P, G> | VectorFeature<M, D, P, G> | S2Feature<M, D, P, G>;

/** Any Vector Geometry type */
export type VectorFeatures<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
> = VectorFeature<M, D, P> | S2Feature<M, D, P>;

/** All major S2JSON types */
export type JSONCollection<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
  G = Geometry<D> | VectorGeometry<D>,
> = FeatureCollection<M, D, P, G> | S2FeatureCollection<M, D, P, G> | Features<M, D, P, G>;
