import {
  childrenIJ,
  contains,
  face,
  fromFace,
  fromID,
  isFace,
  level,
  parent,
  toIJ,
} from '../src/id';
import { describe, expect, it } from 'bun:test';

describe('childrenIJ', () => {
  it('S2', () => {
    const children = childrenIJ('S2', 0, 0, 0, 0);
    expect(children).toEqual([
      288230376151711744n,
      2017612633061982208n,
      864691128455135232n,
      1441151880758558720n,
    ]);
  });

  it('WM', () => {
    const children = childrenIJ('WM', 0, 0, 0, 0);
    expect(children).toEqual([
      288230376151711744n,
      288230376688582656n,
      288230376151711745n,
      288230376688582657n,
    ]);
  });
});

describe('contains', () => {
  it('S2', () => {
    const face = fromFace('S2', 0);
    expect(contains('S2', face, 288230376151711744n)).toBe(true);
    expect(contains('S2', face, 2017612633061982208n)).toBe(true);
    expect(contains('S2', face, 864691128455135232n)).toBe(true);
    expect(contains('S2', face, 1441151880758558720n)).toBe(true);
  });

  it('WM', () => {
    const face = fromFace('WM', 0);
    expect(contains('WM', face, 288230376151711744n)).toBe(true);
    expect(contains('WM', face, 288230376688582656n)).toBe(true);
  });
});

describe('face', () => {
  it('S2', () => {
    expect(face('S2', 288230376151711744n)).toBe(0);
  });
  it('WM', () => {
    expect(face('WM', 288230376151711744n)).toBe(0);
  });
});

describe('fromFace', () => {
  it('S2', () => {
    const face = fromFace('S2', 0);
    expect(face).toBe(1152921504606846976n);
  });

  it('WM', () => {
    const face = fromFace('WM', 0);
    expect(face).toBe(0n);
  });
});

describe('fromID', () => {
  it('WM', () => {
    const id = fromID('WM', 0n);
    expect(id).toEqual([0, 0, 0, 0]);
  });

  it('S2', () => {
    const id = fromID('S2', 0n);
    expect(id).toEqual([0, 0, 0, 0]);
  });
});

describe('isFace', () => {
  it('S2', () => {
    expect(isFace('S2', 288230376151711744n)).toBeFalse();
    expect(isFace('S2', 2017612633061982208n)).toBeFalse();
    expect(isFace('S2', 1152921504606846976n)).toBeTrue();
  });
  it('WM', () => {
    expect(isFace('WM', 288230376151711744n)).toBeFalse();
    expect(isFace('WM', 0n)).toBeTrue();
  });
});

describe('level', () => {
  it('S2', () => {
    expect(level('S2', 1152921504606846976n)).toBe(0);
    expect(level('S2', 288230376151711744n)).toBe(1);
  });

  it('WM', () => {
    expect(level('WM', 0n)).toBe(0);
    expect(level('WM', 288230376151711744n)).toBe(1);
  });
});

describe('parent', () => {
  it('S2', () => {
    expect(parent('S2', 288230376151711744n)).toBe(1152921504606846976n);
  });

  it('WM', () => {
    expect(parent('WM', 288230376151711744n)).toBe(0n);
  });
});

describe('toIJ', () => {
  it('S2', () => {
    expect(toIJ('S2', 288230376151711744n)).toEqual([0, 268435456, 268435456]);
  });
  it('WM', () => {
    expect(toIJ('WM', 288230376151711744n)).toEqual([1, 0, 0]);
  });
});
