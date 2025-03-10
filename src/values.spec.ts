/** Primitive types supported by Properties */
export type Primitive = string | number | boolean | null | undefined;

/** ValueArray Objects are limited to primitives */
export interface ValueArrayObject {
  [key: string]: Primitive;
}

/**
 * When an array is used, it must be an array of the same type.
 * Arrays are also limited to primitives and objects of primitives
 */
export type ValueArray = (Primitive | ValueArrayObject)[] extends (infer U)[] ? U[] : never;

/** Values can have nested objects */
export interface ValueObject {
  [key: string]: Value;
}

/**
 * Supports primitive types `string`, `number`, `boolean`, `null`
 * May be an array of those types, or an object of those types
 * Object keys are always strings, values can be any basic type, an array, or a nested object.
 * Array values must all be the same type.
 */
export type Value = Primitive | ValueArray | ValueObject;

/**
 * Shape of a features properties object
 * NOTE: When designing an interface, you MAY have undefined properties like: { a?: string },
 * but know that they will probably be serialized as null
 */
export type Properties = Record<string, Value>;
/** Shape of a feature's M-Values object */
export type MValue = Properties;
/** Ensure all elements in an array are the same */
type AllEqual<T> = T extends [infer First, ...infer Rest]
  ? Rest extends [First, ...infer _]
    ? AllEqual<Rest>
    : false
  : true;

/** Uniform MValues Shape */
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
