import { Tile } from '../src/tile';
import { expect, test } from 'bun:test';

const SIMPLIFY_MAXZOOM = 16;

test('Tile', () => {
  const tile = new Tile(0n, 'WM');
  expect(tile).toEqual({
    id: 0n,
    projection: 'WM',
    layers: {},
    simplified: false,
  } as Tile);

  tile.addFeature({
    type: 'VectorFeature',
    properties: {},
    geometry: {
      type: 'Point',
      coordinates: { x: 0, y: 0 },
    },
  });

  tile.simplify(3, SIMPLIFY_MAXZOOM);

  expect(tile).toEqual({
    id: 0n,
    projection: 'WM',
    simplified: true,
    layers: {
      default: {
        name: 'default',
        features: [
          {
            type: 'VectorFeature',
            properties: {},
            geometry: {
              type: 'Point',
              coordinates: { x: 0, y: 0 },
            },
          },
        ],
      },
    },
  } as unknown as Tile);
});
