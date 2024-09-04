import { toWM } from './s2';
import { toS2, toUnitScale, toVector } from './wm';

import type {
  Feature,
  JSONCollection,
  Projection,
  S2Feature,
  VectorFeature,
  VectorFeatures,
} from '.';

/**
 * @param projection - output either S2 or WM
 * @param data - the data to convert
 * @param tolerance - optionally specify a tolerance to prepare for future simplification
 * @param maxzoom - optionally specify a maxzoom to prepare for future simplification
 * @param buildBBox - optional - build a bbox for the feature if desired
 * @returns - the converted data
 */
export function convert(
  projection: Projection,
  data: JSONCollection,
  tolerance?: number,
  maxzoom?: number,
  buildBBox?: boolean,
): VectorFeatures[] {
  const res: VectorFeatures[] = [];

  if (data.type === 'Feature') {
    res.push(...convertFeature(projection, data, tolerance, maxzoom, buildBBox));
  } else if (data.type === 'VectorFeature') {
    res.push(...convertVectorFeature(projection, data, tolerance, maxzoom));
  } else if (data.type === 'FeatureCollection') {
    for (const feature of data.features) {
      if (feature.type === 'Feature')
        res.push(...convertFeature(projection, feature, tolerance, maxzoom, buildBBox));
      else res.push(...convertVectorFeature(projection, feature, tolerance, maxzoom));
    }
  } else if (data.type === 'S2Feature') {
    res.push(convertS2Feature(projection, data, tolerance, maxzoom));
  } else if (data.type === 'S2FeatureCollection') {
    for (const feature of data.features) {
      res.push(convertS2Feature(projection, feature, tolerance, maxzoom));
    }
  }

  return res;
}

/**
 * @param projection - either S2 or WM is the end goal feature
 * @param data - input feature data
 * @param tolerance - optionally specify a tolerance to prepare for future simplification
 * @param maxzoom - optionally specify a maxzoom to prepare for future simplification
 * @param buildBBox - optional - build a bbox for the feature if desired
 * @returns - converted feature
 */
function convertFeature(
  projection: Projection,
  data: Feature,
  tolerance?: number,
  maxzoom?: number,
  buildBBox?: boolean,
): VectorFeatures[] {
  if (projection === 'WM') {
    const vf = toVector(data, buildBBox);
    toUnitScale(vf, tolerance, maxzoom);
    return [vf];
  } else {
    return toS2(data, tolerance, maxzoom, buildBBox);
  }
}

/**
 * @param projection - either S2 or WM is the end goal feature
 * @param data - input feature data
 * @param tolerance - optionally specify a tolerance to prepare for future simplification
 * @param maxzoom - optionally specify a maxzoom to prepare for future simplification
 * @returns - converted feature(s)
 */
function convertVectorFeature(
  projection: Projection,
  data: VectorFeature,
  tolerance?: number,
  maxzoom?: number,
): VectorFeatures[] {
  if (projection === 'WM') {
    toUnitScale(data, tolerance, maxzoom);
    return [data];
  } else {
    return toS2(data, tolerance, maxzoom);
  }
}

/**
 * @param projection - either S2 or WM is the end goal feature
 * @param data - input feature data
 * @param tolerance - optionally specify a tolerance to prepare for future simplification
 * @param maxzoom - optionally specify a maxzoom to prepare for future simplification
 * @returns - converted feature
 */
function convertS2Feature(
  projection: Projection,
  data: S2Feature,
  tolerance?: number,
  maxzoom?: number,
): VectorFeatures {
  if (projection === 'WM') {
    const vf = toWM(data);
    toUnitScale(vf, tolerance, maxzoom);
    return vf;
  } else {
    return data;
  }
}
