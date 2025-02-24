import { expect, test } from 'bun:test';

import type { BBOX, BBox, BBox3D, MValue, Properties, VectorPoint } from '../src';

test('VectorPoint', () => {
  const p: VectorPoint = { x: 1, y: 2, z: 3, m: { a: 1, b: 'two' }, t: 5 };
  expect(p.x).toEqual(1);
  expect(p.y).toEqual(2);
  expect(p.z).toEqual(3);
  expect(p.m).toEqual({ a: 1, b: 'two' });
  expect(p.t).toEqual(5);
});

test('BBox', () => {
  const b: BBox = [1, 2, 3, 4];
  expect(b).toEqual([1, 2, 3, 4]);
});

test('BBox3D', () => {
  const b: BBox3D = [1, 2, 3, 4, 5, 6];
  expect(b).toEqual([1, 2, 3, 4, 5, 6]);
});

test('MValue', () => {
  const m: MValue = { a: 1, b: 'two' };
  expect(m).toEqual({ a: 1, b: 'two' });
});

test('Properties', () => {
  const p: Properties = { a: 1, b: 'two' };
  expect(p).toEqual({ a: 1, b: 'two' });
});

test('BBOX', () => {
  const b: BBOX = [1, 2, 3, 4];
  expect(b).toEqual([1, 2, 3, 4]);
  const b2: BBOX = [1, 2, 3, 4, 5, 6];
  expect(b2).toEqual([1, 2, 3, 4, 5, 6]);
});
