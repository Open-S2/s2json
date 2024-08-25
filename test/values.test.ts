import { describe, expect, it } from 'bun:test';

import type { Primitive } from '../src';

describe('primitive', () => {
  it('can be `string | number | boolean | null`', () => {
    expect<Primitive>('string').toBe('string');
    expect<Primitive>(1).toBe(1);
    expect<Primitive>(true).toBe(true);
    expect<Primitive>(null).toBe(null);
    // @ts-expect-error - undefined is not a primitive
    const und: Primitive = undefined;
    expect(und).toBeUndefined();
  });
});
