import { buildSqDists, simplify } from '../src/simplify';
import { expect, test } from 'bun:test';

import type { VectorLineStringGeometry } from '../src';

const SIMPLIFY_MAXZOOM = 16;

test('LineString', () => {
  const lineString: VectorLineStringGeometry = {
    type: 'LineString',
    coordinates: [
      { x: 0.25, y: 0.25 },
      { x: 0.75, y: 0.25 },
      { x: 0.75, y: 0.75 },
      { x: 0.25, y: 0.75 },
    ],
    vecBBox: [0.25, 0.25, 0.75, 0.75],
  };

  buildSqDists(lineString, 3, SIMPLIFY_MAXZOOM);

  expect(lineString).toEqual({
    type: 'LineString',
    coordinates: [
      { x: 0.25, y: 0.25, t: 1 },
      { x: 0.75, y: 0.25, t: 0.125 },
      { x: 0.75, y: 0.75, t: 0.25 },
      { x: 0.25, y: 0.75, t: 1 },
    ],
    vecBBox: [0.25, 0.25, 0.75, 0.75],
  });

  simplify(lineString, 3, 0, SIMPLIFY_MAXZOOM);
  expect(lineString).toEqual({
    type: 'LineString',
    coordinates: [
      { x: 0.25, y: 0.25, t: 1 },
      { x: 0.75, y: 0.25, t: 0.125 },
      { x: 0.75, y: 0.75, t: 0.25 },
      { x: 0.25, y: 0.75, t: 1 },
    ],
    vecBBox: [0.25, 0.25, 0.75, 0.75],
  });
});
