import Ajv from 'ajv';
import { describe, expect, it } from 'bun:test';

import * as schema from '../src/s2json.schema.json'; // Path to your JSON schema file

import type { Feature } from '../src';

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
