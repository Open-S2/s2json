/** Primitive types supported by Properties */
type Primitive = string | number | boolean | null;

/**
 * When an array is used, it must be an array of the same type.
 * Arrays are also limited to primitives and objects of primitives
 */
type ValueArray =
  Array<Primitive | { [key: string]: Primitive }> extends Array<infer U> ? U[] : never;

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
/** LineString Properties Shape */
export type LineStringMValues = MValue[];
/** MultiLineString Properties Shape */
export type MultiLineStringMValues = MValue[][];
/** Polygon MValues Shape */
export type PolygonMValues = MValue[][];
/** MultiPolygon MValues Shape */
export type MultiPolygonMValues = MValue[][][];

/** All possible M-Value shapes */
export type MValues =
  | LineStringMValues
  | MultiLineStringMValues
  | PolygonMValues
  | MultiPolygonMValues;
