import { bboxOverlap, pointOverlap } from '../src/bbox';
import { describe, expect, it } from 'bun:test';

describe('pointOverlap', () => {
  it('check if point is within bbox', () => {
    expect(pointOverlap([0, 0, 1, 1], [0.5, 0.5])).toBeTrue();
  });
  it('check if point is not within bbox', () => {
    expect(pointOverlap([0, 0, 1, 1], [2, 2])).toBeFalse();
  });
});

describe('bboxOverlap', () => {
  it('no overlap returns undefined', () => {
    expect(bboxOverlap([0, 0, 1, 1], [2, 2, 3, 3])).toBeUndefined();
  });
  it('overlap returns bbox', () => {
    expect(bboxOverlap([0, 0, 1, 1], [0.5, 0.5, 1.5, 1.5])).toEqual([0.5, 0.5, 1, 1]);
  });
});
