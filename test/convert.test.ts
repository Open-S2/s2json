import { convert } from '../src/convert';
import { describe, expect, it } from 'bun:test';

import type {
  Feature,
  FeatureCollection,
  S2Feature,
  S2FeatureCollection,
  VectorFeature,
} from '../src';

// FeatureCollection | S2FeatureCollection | Feature | VectorFeature | S2Feature

describe('convert point', () => {
  it('WM', () => {
    const feature: Feature = {
      type: 'Feature',
      properties: { a: 1 },
      geometry: {
        type: 'Point',
        coordinates: [0, 0],
      },
    };
    const vectorFeature: VectorFeature = {
      type: 'VectorFeature',
      properties: { b: 2 },
      geometry: {
        type: 'Point',
        is3D: true,
        coordinates: { x: 45, y: 45, z: 10, m: { c: 3 } },
        bbox: [0.5, 0.5, 0.75, 0.75],
      },
    };
    const s2Feature: S2Feature = {
      type: 'S2Feature',
      properties: { c: 3 },
      geometry: {
        type: 'Point',
        is3D: true,
        coordinates: { x: 45, y: 45, z: 10, m: { d: 4 } },
        bbox: [0, 0, 1, 1],
      },
      face: 0,
    };
    const featureCollection: FeatureCollection = {
      type: 'FeatureCollection',
      features: [
        {
          type: 'Feature',
          properties: { a: 1 },
          geometry: {
            type: 'Point',
            coordinates: [0, 0],
            bbox: [0.1, 0.1, 0.2, 0.2],
          },
        },
        {
          type: 'VectorFeature',
          properties: { b: 2 },
          geometry: {
            type: 'Point',
            is3D: true,
            coordinates: { x: 45, y: 45, z: 10, m: { c: 3 } },
            bbox: [0.5, 0.5, 0.75, 0.75],
          },
        },
      ],
    };
    const s2FeatureCollection: S2FeatureCollection = {
      type: 'S2FeatureCollection',
      features: [
        {
          type: 'S2Feature',
          properties: { c: 3 },
          geometry: {
            is3D: true,
            type: 'Point',
            coordinates: { x: 45, y: 45, z: 10, m: { d: 4 } },
            bbox: [0, 0, 1, 1],
          },
          face: 0,
        },
      ],
      faces: [0],
    };

    const res1 = convert('WM', feature, 3, 14, true);
    const res2 = convert('WM', vectorFeature, 3, 14);
    const res3 = convert('WM', s2Feature, 3, 14);
    const res4 = convert('WM', featureCollection, 3, 14);
    const res5 = convert('WM', s2FeatureCollection, 3, 14);

    expect(res1).toEqual([
      {
        geometry: {
          bbox: [0, 0, 0, 0],
          is3D: false,
          coordinates: {
            m: undefined,
            x: 0.5,
            y: 0.5,
            z: undefined,
          },
          type: 'Point',
          vecBBox: [0.5, 0.5, 0.5, 0.5],
        },
        id: undefined,
        metadata: undefined,
        properties: {
          a: 1,
        },
        type: 'VectorFeature',
      },
    ]);
    expect(res2).toEqual([
      {
        geometry: {
          bbox: [0.5, 0.5, 0.75, 0.75],
          is3D: true,
          coordinates: {
            m: { c: 3 },
            x: 0.625,
            y: 0.35972503691520497,
            z: 10,
          },
          type: 'Point',
          vecBBox: [0.625, 0.35972503691520497, 0.625, 0.35972503691520497, 10, 10],
        },
        properties: { b: 2 },
        type: 'VectorFeature',
      },
    ]);
    expect(res3).toEqual([
      {
        geometry: {
          bbox: [0, 0, 1, 1],
          is3D: true,
          coordinates: {
            m: { d: 4 },
            x: 0.7499410464492606,
            y: 0.35972504463587185,
            z: 10,
          },
          type: 'Point',
          vecBBox: [
            0.7499410464492606, 0.35972504463587185, 0.7499410464492606, 0.35972504463587185, 10,
            10,
          ],
        },
        id: undefined,
        metadata: undefined,
        properties: {
          c: 3,
        },
        type: 'VectorFeature',
      },
    ]);
    expect(res4).toEqual([
      {
        geometry: {
          bbox: [0.1, 0.1, 0.2, 0.2],
          is3D: false,
          coordinates: {
            m: undefined,
            x: 0.5,
            y: 0.5,
            z: undefined,
          },
          type: 'Point',
          vecBBox: [0.5, 0.5, 0.5, 0.5],
        },
        id: undefined,
        metadata: undefined,
        properties: {
          a: 1,
        },
        type: 'VectorFeature',
      },
      {
        geometry: {
          bbox: [0.5, 0.5, 0.75, 0.75],
          is3D: true,
          coordinates: {
            m: {
              c: 3,
            },
            x: 0.625,
            y: 0.35972503691520497,
            z: 10,
          },
          type: 'Point',
          vecBBox: [0.625, 0.35972503691520497, 0.625, 0.35972503691520497, 10, 10],
        },
        properties: {
          b: 2,
        },
        type: 'VectorFeature',
      },
    ]);
    expect(res5).toEqual([
      {
        geometry: {
          bbox: [0, 0, 1, 1],
          is3D: true,
          coordinates: {
            m: {
              d: 4,
            },
            x: 0.7499410464492606,
            y: 0.35972504463587185,
            z: 10,
          },
          type: 'Point',
          vecBBox: [
            0.7499410464492606, 0.35972504463587185, 0.7499410464492606, 0.35972504463587185, 10,
            10,
          ],
        },
        id: undefined,
        metadata: undefined,
        properties: {
          c: 3,
        },
        type: 'VectorFeature',
      },
    ]);
  });

  it('S2', () => {
    const feature: Feature = {
      type: 'Feature',
      properties: { a: 1 },
      geometry: {
        type: 'Point',
        coordinates: [0, 0],
        bbox: [0.1, 0.1, 0.2, 0.2],
      },
    };
    const vectorFeature: VectorFeature = {
      type: 'VectorFeature',
      properties: { b: 2 },
      geometry: {
        type: 'Point',
        is3D: true,
        coordinates: { x: 45, y: 45, z: 10, m: { c: 3 } },
        bbox: [0.5, 0.5, 0.75, 0.75],
      },
    };
    const s2Feature: S2Feature = {
      type: 'S2Feature',
      properties: { c: 3 },
      geometry: {
        type: 'Point',
        is3D: true,
        coordinates: { x: 45, y: 45, z: 10, m: { d: 4 } },
        bbox: [0, 0, 1, 1],
      },
      face: 0,
    };
    const featureCollection: FeatureCollection = {
      type: 'FeatureCollection',
      features: [
        {
          type: 'Feature',
          properties: { a: 1 },
          geometry: {
            type: 'Point',
            coordinates: [45, 22],
          },
        },
        {
          type: 'VectorFeature',
          properties: { b: 2 },
          geometry: {
            type: 'Point',
            is3D: true,
            coordinates: { x: 45, y: 45, z: 10, m: { c: 3 } },
            bbox: [0.5, 0.5, 0.75, 0.75],
          },
        },
      ],
    };
    const s2FeatureCollection: S2FeatureCollection = {
      type: 'S2FeatureCollection',
      features: [
        {
          type: 'S2Feature',
          properties: { c: 3 },
          geometry: {
            type: 'Point',
            is3D: true,
            coordinates: { x: 45, y: 45, z: 10, m: { d: 4 } },
            bbox: [0, 0, 1, 1],
          },
          face: 0,
        },
      ],
      faces: [0],
    };

    const res1 = convert('S2', feature, 3, 14);
    const res2 = convert('S2', vectorFeature, 3, 14);
    const res3 = convert('S2', s2Feature, 3, 14);
    const res4 = convert('S2', featureCollection, 3, 14, true);
    const res5 = convert('S2', s2FeatureCollection, 3, 14);

    expect(res1).toEqual([
      {
        face: 0,
        geometry: {
          bbox: [0.1, 0.1, 0.2, 0.2],
          is3D: false,
          coordinates: {
            m: undefined,
            x: 0.5,
            y: 0.5,
            z: undefined,
          },
          type: 'Point',
          vecBBox: [0.5, 0.5, 0.5, 0.5],
        },
        id: undefined,
        metadata: undefined,
        properties: {
          a: 1,
        },
        type: 'S2Feature',
      },
    ]);

    expect(res2).toEqual([
      {
        face: 2,
        geometry: {
          bbox: [0.5, 0.5, 0.75, 0.75],
          is3D: true,
          coordinates: {
            m: { c: 3 },
            x: 0.11663705879751174,
            y: 0.11663705879751174,
            z: 10,
          },
          type: 'Point',
          vecBBox: [
            0.11663705879751174, 0.11663705879751174, 0.11663705879751174, 0.11663705879751174, 10,
            10,
          ],
        },
        id: undefined,
        metadata: undefined,
        properties: { b: 2 },
        type: 'S2Feature',
      },
    ]);

    expect(res3).toEqual([
      {
        face: 0,
        geometry: {
          bbox: [0, 0, 1, 1],
          is3D: true,
          coordinates: {
            m: {
              d: 4,
            },
            x: 45,
            y: 45,
            z: 10,
          },
          type: 'Point',
        },
        properties: {
          c: 3,
        },
        type: 'S2Feature',
      },
    ]);

    expect(res4).toEqual([
      {
        face: 0,
        geometry: {
          bbox: [45, 22, 45, 22],
          is3D: false,
          coordinates: {
            m: undefined,
            x: 0.9999999999999999,
            y: 0.8237320717914717,
            z: undefined,
          },
          type: 'Point',
          vecBBox: [0.9999999999999999, 0.8237320717914717, 0.9999999999999999, 0.8237320717914717],
        },
        id: undefined,
        metadata: undefined,
        properties: {
          a: 1,
        },
        type: 'S2Feature',
      },
      {
        face: 2,
        geometry: {
          bbox: [0.5, 0.5, 0.75, 0.75],
          is3D: true,
          coordinates: {
            m: {
              c: 3,
            },
            x: 0.11663705879751174,
            y: 0.11663705879751174,
            z: 10,
          },
          type: 'Point',
          vecBBox: [
            0.11663705879751174, 0.11663705879751174, 0.11663705879751174, 0.11663705879751174, 10,
            10,
          ],
        },
        id: undefined,
        metadata: undefined,
        properties: {
          b: 2,
        },
        type: 'S2Feature',
      },
    ]);

    expect(res5).toEqual([
      {
        face: 0,
        geometry: {
          bbox: [0, 0, 1, 1],
          is3D: true,
          coordinates: {
            m: { d: 4 },
            x: 45,
            y: 45,
            z: 10,
          },
          type: 'Point',
        },
        properties: { c: 3 },
        type: 'S2Feature',
      },
    ]);
  });
});
