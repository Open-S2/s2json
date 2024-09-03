import Ajv from 'ajv';
import { describe, expect, it } from 'bun:test';

import schema from '../src/s2json.schema.json';

import type { Feature, S2Feature, VectorFeature } from '../src';

const ajv = new Ajv();
const validate = ajv.compile(schema);

describe('feature', () => {
  it('feature point', () => {
    const validFeature: Feature = {
      type: 'Feature',
      geometry: {
        type: 'Point',
        coordinates: [-75.165222, 39.952583],
      },
      properties: {},
    };
    expect(validate(validFeature)).toBeTrue();

    const validFeatureWithOptionals: Feature = {
      id: 0,
      type: 'Feature',
      geometry: {
        type: 'Point',
        coordinates: [-75.165222, 39.952583],
        bbox: [-75.165222, 39.952583, -75.165222, 39.952583],
        mValues: { a: 1 },
      },
      properties: {
        name: 'Home',
      },
    };
    expect(validate(validFeatureWithOptionals)).toBeTrue();

    const badID: Feature = {
      id: -2,
      type: 'Feature',
      geometry: {
        type: 'Point',
        coordinates: [-75.165222, 39.952583],
      },
      properties: {},
    };
    expect(validate(badID)).toBeFalse();

    // @ts-expect-error - no properties
    const noProperties: Feature = {
      type: 'Feature',
      geometry: {
        type: 'Point',
        coordinates: [-75.165222, 39.952583],
      },
    };
    expect(validate(noProperties)).toBeFalse();

    // @ts-expect-error - no geometry
    const noGeometry: Feature = {
      type: 'Feature',
      properties: {},
    };
    expect(validate(noGeometry)).toBeFalse();

    // @ts-expect-error - no type
    const noType: Feature = {
      geometry: {
        type: 'Point',
        coordinates: [-75.165222, 39.952583],
      },
      properties: {},
    };
    expect(validate(noType)).toBeFalse();

    const badType: Feature = {
      // @ts-expect-error - bad type
      type: 'Point',
      geometry: {
        type: 'Point',
        coordinates: [-75.165222, 39.952583],
      },
      properties: {},
    };
    expect(validate(badType)).toBeFalse();

    const badGeometryType: Feature = {
      type: 'Feature',
      geometry: {
        // @ts-expect-error - bad geometry
        type: 'Feature',
        coordinates: [-75.165222, 39.952583],
      },
      properties: {},
    };
    expect(validate(badGeometryType)).toBeFalse();

    const badGeometryCoordinates: Feature = {
      type: 'Feature',
      geometry: {
        type: 'Point',
        // @ts-expect-error - bad geometry
        coordinates: [[-75.165222, 39.952583]],
      },
      properties: {},
    };
    expect(validate(badGeometryCoordinates)).toBeFalse();

    const badGeometryBBox: Feature = {
      type: 'Feature',
      geometry: {
        type: 'Point',
        coordinates: [-75.165222, 39.952583],
        // @ts-expect-error - bad geometry
        bbox: [-75.165222, 39.952583, -75.165222, 39.952583, -75.165222],
      },
      properties: {},
    };
    expect(validate(badGeometryBBox)).toBeFalse();

    const badGeometryMValues: Feature = {
      type: 'Feature',
      // @ts-expect-error - bad geometry
      geometry: {
        type: 'Point',
        coordinates: [-75.165222, 39.952583],
        mValues: [{ a: 1 }],
      },
      properties: {},
    };
    expect(validate(badGeometryMValues)).toBeFalse();
  });
});

describe('vector feature', () => {
  it('feature point', () => {
    const validFeature: VectorFeature = {
      type: 'VectorFeature',
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
      properties: {},
    };
    expect(validate(validFeature)).toBeTrue();

    const validFeatureWithOptionals: VectorFeature = {
      id: 0,
      type: 'VectorFeature',
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583, m: { a: 1 } },
        bbox: [-75.165222, 39.952583, -75.165222, 39.952583],
      },
      properties: {
        name: 'Home',
      },
    };
    expect(validate(validFeatureWithOptionals)).toBeTrue();

    const badID: VectorFeature = {
      id: -2,
      type: 'VectorFeature',
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
      properties: {},
    };
    expect(validate(badID)).toBeFalse();

    // @ts-expect-error - no properties
    const noProperties: VectorFeature = {
      type: 'VectorFeature',
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
    };
    expect(validate(noProperties)).toBeFalse();

    // @ts-expect-error - no geometry
    const noGeometry: VectorFeature = {
      type: 'VectorFeature',
      properties: {},
    };
    expect(validate(noGeometry)).toBeFalse();

    // @ts-expect-error - no type
    const noType: VectorFeature = {
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
      properties: {},
    };
    expect(validate(noType)).toBeFalse();

    const badType: VectorFeature = {
      // @ts-expect-error - bad type
      type: 'Point',
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
      properties: {},
    };
    expect(validate(badType)).toBeFalse();

    const badGeometryType: VectorFeature = {
      type: 'VectorFeature',
      geometry: {
        // @ts-expect-error - bad geometry
        type: 'Feature',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
      properties: {},
    };
    expect(validate(badGeometryType)).toBeFalse();

    const badGeometryCoordinates: VectorFeature = {
      type: 'VectorFeature',
      // @ts-expect-error - bad geometry
      geometry: {
        type: 'Point',
        coordinates: [{ x: -75.165222, y: 39.952583 }],
      },
      properties: {},
    };
    expect(validate(badGeometryCoordinates)).toBeFalse();

    const badGeometryBBox: VectorFeature = {
      type: 'VectorFeature',
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
        // @ts-expect-error - bad bounding box
        bbox: [-75.165222, 39.952583, -75.165222, 39.952583, -75.165222],
      },
      properties: {},
    };
    expect(validate(badGeometryBBox)).toBeFalse();

    const badGeometryMValues: VectorFeature = {
      type: 'VectorFeature',
      geometry: {
        type: 'Point',
        // @ts-expect-error - bad m-value
        coordinates: { x: -75.165222, y: 39.952583, m: [{ a: 1 }] },
      },
      properties: {},
    };
    expect(validate(badGeometryMValues)).toBeFalse();
  });
});

describe('s2 feature', () => {
  it('feature point', () => {
    const validFeature: S2Feature = {
      type: 'S2Feature',
      face: 0,
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
      properties: {},
    };
    expect(validate(validFeature)).toBeTrue();

    const validFeatureWithOptionals: S2Feature = {
      id: 0,
      face: 0,
      type: 'S2Feature',
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583, m: { a: 1 } },
        bbox: [-75.165222, 39.952583, -75.165222, 39.952583],
      },
      properties: {
        name: 'Home',
      },
    };
    expect(validate(validFeatureWithOptionals)).toBeTrue();

    const badID: S2Feature = {
      // bad id
      id: -2,
      face: 0,
      type: 'S2Feature',
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
      properties: {},
    };
    expect(validate(badID)).toBeFalse();

    // @ts-expect-error - no properties
    const noProperties: S2Feature = {
      type: 'S2Feature',
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
    };
    expect(validate(noProperties)).toBeFalse();

    // @ts-expect-error - no geometry
    const noGeometry: S2Feature = {
      type: 'S2Feature',
      properties: {},
    };
    expect(validate(noGeometry)).toBeFalse();

    // @ts-expect-error - no type
    const noType: S2Feature = {
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
      properties: {},
    };
    expect(validate(noType)).toBeFalse();

    const badType: S2Feature = {
      // @ts-expect-error - bad type
      type: 'Point',
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
      properties: {},
    };
    expect(validate(badType)).toBeFalse();

    const badGeometryType: S2Feature = {
      type: 'S2Feature',
      geometry: {
        // @ts-expect-error - bad geometry
        type: 'Feature',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
      properties: {},
    };
    expect(validate(badGeometryType)).toBeFalse();

    const badGeometryCoordinates: S2Feature = {
      type: 'S2Feature',
      // @ts-expect-error - bad geometry
      geometry: {
        type: 'Point',
        coordinates: [{ x: -75.165222, y: 39.952583 }],
      },
      properties: {},
    };
    expect(validate(badGeometryCoordinates)).toBeFalse();

    const badGeometryBBox: S2Feature = {
      type: 'S2Feature',
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
        // @ts-expect-error - bad bounding box
        bbox: [-75.165222, 39.952583, -75.165222, 39.952583, -75.165222],
      },
      properties: {},
    };
    expect(validate(badGeometryBBox)).toBeFalse();

    const badGeometryMValues: S2Feature = {
      type: 'S2Feature',
      face: 0,
      geometry: {
        type: 'Point',
        // @ts-expect-error - bad m-value
        coordinates: { x: -75.165222, y: 39.952583, m: [{ a: 1 }] },
      },
      properties: {},
    };
    expect(validate(badGeometryMValues)).toBeFalse();

    const badFace: S2Feature = {
      id: 0,
      // @ts-expect-error - bad face
      face: 10,
      type: 'S2Feature',
      geometry: {
        type: 'Point',
        coordinates: { x: -75.165222, y: 39.952583 },
      },
      properties: {},
    };
    expect(validate(badFace)).toBeFalse();
  });
});
