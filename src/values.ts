/** Primitive types supported by Properties */
export type Primitive = string | number | boolean | null;

/**
 * When an array is used, it must be an array of the same type.
 * Arrays are also limited to primitives and objects of primitives
 */
export type ValueArray =
  Array<Primitive | { [key: string]: Primitive }> extends (infer U)[] ? U[] : never;

/**
 * Supports primitive types `string`, `number`, `boolean`, `null`
 * May be an array of those types, or an object of those types
 * Object keys are always strings, values can be any basic type, an array, or a nested object.
 * Array values must all be the same type.
 */
export type Value = Primitive | ValueArray | { [key: string]: Value };

/** Shape of a features properties object */
export type Properties = Record<string, Value>;
/** Shape of a feature's M-Values object */
export type MValue = Properties;
/**
 *
 */
type AllEqual<T> = T extends [infer First, ...infer Rest]
  ? Rest extends [First, ...infer _]
    ? AllEqual<Rest>
    : false
  : true;

/**
 *
 */
type UniformArray<M extends MValue = MValue> = AllEqual<M[]> extends true ? M[] : never;
/** LineString Properties Shape */
export type LineStringMValues<M extends MValue = MValue> = UniformArray<M>;
/** MultiLineString Properties Shape */
export type MultiLineStringMValues<M extends MValue = MValue> = UniformArray<M>[];
/** Polygon MValues Shape */
export type PolygonMValues<M extends MValue = MValue> = UniformArray<M>[];
/** MultiPolygon MValues Shape */
export type MultiPolygonMValues<M extends MValue = MValue> = UniformArray<M>[][];

/** All possible M-Value shapes */
export type MValues =
  | LineStringMValues
  | MultiLineStringMValues
  | PolygonMValues
  | MultiPolygonMValues;
