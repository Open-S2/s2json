import { VectorPoint } from '../src';
import { expect, test } from 'bun:test';

test('VectorPoint', () => {
  const p: VectorPoint = { x: 1, y: 2, z: 3, m: { a: 1, b: 'two' }, t: 5 };
  expect(p.x).toEqual(1);
  expect(p.y).toEqual(2);
  expect(p.z).toEqual(3);
  expect(p.m).toEqual({ a: 1, b: 'two' });
  expect(p.t).toEqual(5);
});
