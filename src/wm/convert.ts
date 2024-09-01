import { buildSqDists } from '../';
import { clipLine } from './clip';
import { extendBBox, fromPoint, mergeBBoxes } from '../bbox';
import { fromLonLat, toST } from '../s2/s2Point';

import type {
  BBOX,
  Face,
  Feature,
  Geometry,
  MValue,
  Point,
  Point3D,
  S2Feature,
  STPoint,
  VectorCoordinates,
  VectorFeature,
  VectorGeometry,
  VectorLineString,
  VectorLineStringGeometry,
  VectorMultiLineStringGeometry,
  VectorMultiPointGeometry,
  VectorMultiPolygonGeometry,
  VectorPoint,
  VectorPointGeometry,
  VectorPolygon,
  VectorPolygonGeometry,
} from '../';

/**
 * Convet a GeoJSON Feature to an S2Feature
 * @param data - GeoJSON Feature
 * @returns - S2Feature
 */
export function toS2(data: Feature | VectorFeature): S2Feature[] {
  const { id, properties, metadata } = data;
  const res: S2Feature[] = [];
  const vectorGeo = data.type === 'VectorFeature' ? data.geometry : convertGeometry(data.geometry);
  for (const { geometry, face } of convertVectorGeometry(vectorGeo)) {
    res.push({
      id,
      type: 'S2Feature',
      face,
      properties,
      metadata,
      geometry,
    });
  }

  return res;
}

/**
 * Convert a GeoJSON Feature to a GeoJSON Vector Feature
 * @param data - GeoJSON Feature
 * @returns - GeoJson Vector Feature
 */
export function toVector(data: Feature): VectorFeature {
  const { id, properties, metadata } = data;
  const vectorGeo = convertGeometry(data.geometry);
  return {
    id,
    type: 'VectorFeature',
    properties,
    metadata,
    geometry: vectorGeo,
  };
}

/**
 * Mutate a GeoJSON Point to a GeoJson Vector Point
 * @param point - GeoJSON flat Point
 * @param m - optional m-value
 * @returns - GeoJson Vector Point
 */
function convertPoint(point: Point | Point3D, m?: MValue): VectorPoint {
  return { x: point[0], y: point[1], z: point[2], m };
}

/**
 * Convert a GeoJSON Geometry to an Vector Geometry
 * @param geometry - GeoJSON Geometry
 * @returns - GeoJson Vector Geometry
 */
function convertGeometry(geometry: Geometry): VectorGeometry {
  const { type, coordinates: coords, mValues, bbox } = geometry;

  let coordinates: VectorCoordinates;
  if (type === 'Point' || type === 'Point3D') coordinates = convertPoint(coords, mValues);
  else if (type === 'MultiPoint' || type === 'MultiPoint3D')
    coordinates = coords.map((point, i) => convertPoint(point, mValues?.[i]));
  else if (type === 'LineString' || type === 'LineString3D')
    coordinates = coords.map((point, i) => convertPoint(point, mValues?.[i]));
  else if (type === 'MultiLineString' || type === 'MultiLineString3D')
    coordinates = coords.map((line, i) =>
      line.map((point, j) => convertPoint(point, mValues?.[i]?.[j])),
    );
  else if (type === 'Polygon' || type === 'Polygon3D')
    coordinates = coords.map((line, i) =>
      line.map((point, j) => convertPoint(point, mValues?.[i]?.[j])),
    );
  else if (type === 'MultiPolygon' || type === 'MultiPolygon3D')
    coordinates = coords.map((polygon, i) =>
      polygon.map((line, j) => line.map((point, k) => convertPoint(point, mValues?.[i]?.[j]?.[k]))),
    );
  else {
    throw new Error('Invalid GeoJSON type');
  }
  /// @ts-expect-error - coordinates complains, but the way this is all written is simpler
  return { type: type.replace('3D', ''), coordinates, bbox };
}

/** The resultant geometry after conversion */
export type ConvertedGeometry = { geometry: VectorGeometry; face: Face }[];

/**
 * Underlying conversion mechanic to move GeoJSON Geometry to S2Geometry
 * @param geometry - GeoJSON Geometry
 * @param tolerance - if provided, geometry will be prepared for simplification by this tolerance
 * @param maxzoom - if provided, geometry will be prepared for simplification up to this zoom
 * @returns - S2Geometry
 */
function convertVectorGeometry(
  geometry: VectorGeometry,
  tolerance?: number,
  maxzoom?: number,
): ConvertedGeometry {
  const { type } = geometry;
  let cGeo: ConvertedGeometry;
  if (type === 'Point') cGeo = convertGeometryPoint(geometry);
  else if (type === 'MultiPoint') cGeo = convertGeometryMultiPoint(geometry);
  else if (type === 'LineString') cGeo = convertGeometryLineString(geometry);
  else if (type === 'MultiLineString') cGeo = convertGeometryMultiLineString(geometry);
  else if (type === 'Polygon') cGeo = convertGeometryPolygon(geometry);
  else if (type === 'MultiPolygon') cGeo = convertGeometryMultiPolygon(geometry);
  else {
    throw new Error('Either the conversion is not yet supported or Invalid S2Geometry type.');
  }
  if (tolerance !== undefined)
    for (const { geometry } of cGeo) buildSqDists(geometry, tolerance, maxzoom);
  return cGeo;
}

/**
 * @param geometry - GeoJSON PointGeometry
 * @returns - S2 PointGeometry
 */
function convertGeometryPoint(geometry: VectorPointGeometry): ConvertedGeometry {
  const { type, coordinates, bbox } = geometry;
  const { x: lon, y: lat, z, m } = coordinates;
  const [face, s, t] = toST(fromLonLat(lon, lat));
  const vecBBox = fromPoint({ x: s, y: t, z });
  return [{ face, geometry: { type, coordinates: { x: s, y: t, z, m }, bbox, vecBBox } }];
}

/**
 * @param geometry - GeoJSON PointGeometry
 * @returns - S2 PointGeometry
 */
function convertGeometryMultiPoint(geometry: VectorMultiPointGeometry): ConvertedGeometry {
  const { coordinates, bbox } = geometry;
  return coordinates.flatMap((coordinates) =>
    convertGeometryPoint({ type: 'Point', coordinates, bbox }),
  );
}

/**
 * @param geometry - GeoJSON LineStringGeometry
 * @returns - S2 LineStringGeometry
 */
function convertGeometryLineString(geometry: VectorLineStringGeometry): ConvertedGeometry {
  const { type, coordinates, bbox } = geometry;

  return convertLineString(coordinates, false).map(({ face, line, offset, vecBBox }) => {
    return { face, geometry: { type, coordinates: line, bbox, offset, vecBBox } };
  });
}

/**
 * @param geometry - GeoJSON MultiLineStringGeometry
 * @returns - S2 MultiLineStringGeometry
 */
function convertGeometryMultiLineString(
  geometry: VectorMultiLineStringGeometry,
): ConvertedGeometry {
  const { coordinates, bbox } = geometry;
  return coordinates
    .flatMap((line) => convertLineString(line, false))
    .map(({ face, line, offset, vecBBox }) => ({
      face,
      geometry: { type: 'LineString', coordinates: line, bbox, offset, vecBBox },
    }));
}

/**
 * @param geometry - GeoJSON PolygonGeometry
 * @returns - S2 PolygonGeometry
 */
function convertGeometryPolygon(geometry: VectorPolygonGeometry): ConvertedGeometry {
  const { type, coordinates, bbox } = geometry;
  const res: ConvertedGeometry = [];

  // conver all lines
  const outerRing = convertLineString(coordinates[0], true);
  const innerRings = coordinates.slice(1).flatMap((line) => convertLineString(line, true));

  // for each face, build a new polygon
  for (const { face, line, offset, vecBBox: polyBBox } of outerRing) {
    const polygon: VectorPolygon = [line];
    const polygonOffsets = [offset];
    for (const { face: innerFace, line: innerLine, offset: innerOffset, vecBBox } of innerRings) {
      if (innerFace === face) {
        polygon.push(innerLine);
        polygonOffsets.push(innerOffset);
        mergeBBoxes(polyBBox, vecBBox);
      }
    }

    res.push({
      face,
      geometry: { type, coordinates: polygon, bbox, offset: polygonOffsets, vecBBox: polyBBox },
    });
  }

  return res;
}

/**
 * @param geometry - GeoJSON MultiPolygonGeometry
 * @returns - S2 MultiPolygonGeometry
 */
function convertGeometryMultiPolygon(geometry: VectorMultiPolygonGeometry): ConvertedGeometry {
  const { coordinates, bbox, offset } = geometry;
  return coordinates.flatMap((polygon, i) =>
    convertGeometryPolygon({ type: 'Polygon', coordinates: polygon, bbox, offset: offset?.[i] }),
  );
}

/** LineString converted from WM to S2 */
interface ConvertedLineString {
  face: Face;
  line: VectorLineString;
  offset: number;
  vecBBox: BBOX;
}

/**
 * @param line - GeoJSON LineString
 * @param isPolygon - true if the line originates from a polygon
 * @returns - S2 LineStrings clipped to it's 0->1 coordinate system
 */
function convertLineString(line: VectorLineString, isPolygon: boolean): ConvertedLineString[] {
  const res: ConvertedLineString[] = [];
  // first re-project all the coordinates to S2
  const newGeometry: STPoint[] = [];
  for (const { x: lon, y: lat, z, m } of line) {
    const [face, s, t] = toST(fromLonLat(lon, lat));
    newGeometry.push({ face, s, t, z, m });
  }
  // find all the faces that exist in the line
  const faces = new Set<Face>();
  newGeometry.forEach(({ face }) => faces.add(face));
  // for each face, build a line
  for (const face of faces) {
    const line: VectorLineString = [];
    for (const stPoint of newGeometry) line.push(stPointToFace(face, stPoint));
    const clippedLines = clipLine(line, [0, 0, 1, 1], isPolygon);
    for (const { line, offset, vecBBox } of clippedLines) res.push({ face, line, offset, vecBBox });
  }

  return res;
}

/**
 * Reproject GeoJSON geometry coordinates from lon-lat to a 0->1 coordinate system in place
 * @param feature - input GeoJSON
 * @param tolerance - if provided, geometry will be prepared for simplification by this tolerance
 * @param maxzoom - if provided,
 */
export function toUnitScale(feature: VectorFeature, tolerance?: number, maxzoom?: number): void {
  const { geometry } = feature;
  const { type, coordinates } = geometry;
  if (type === 'Point') projectPoint(coordinates, geometry);
  else if (type === 'MultiPoint') coordinates.map((p) => projectPoint(p, geometry));
  else if (type === 'LineString') coordinates.map((p) => projectPoint(p, geometry));
  else if (type === 'MultiLineString')
    coordinates.map((l) => l.map((p) => projectPoint(p, geometry)));
  else if (type === 'Polygon') coordinates.map((l) => l.map((p) => projectPoint(p, geometry)));
  else if (type === 'MultiPolygon')
    coordinates.map((p) => p.map((l) => l.map((p) => projectPoint(p, geometry))));
  else {
    throw new Error('Either the conversion is not yet supported or Invalid S2Geometry type.');
  }
  if (tolerance !== undefined) buildSqDists(geometry, tolerance, maxzoom);
}

/**
 * Project a point from lon-lat to a 0->1 coordinate system in place
 * @param input - input point
 * @param geo - input geometry (used to update the bbox)
 */
function projectPoint(input: VectorPoint, geo: VectorGeometry): void {
  const { x, y } = input;
  const sin = Math.sin((y * Math.PI) / 180);
  const y2 = 0.5 - (0.25 * Math.log((1 + sin) / (1 - sin))) / Math.PI;
  input.x = x / 360 + 0.5;
  input.y = y2 < 0 ? 0 : y2 > 1 ? 1 : y2;
  // update bbox
  geo.vecBBox = extendBBox(geo.vecBBox, input);
}

/**
 * @param targetFace - face you want to project to
 * @param stPoint - the point you want to project
 * @returns - the projected point
 */
function stPointToFace(targetFace: Face, stPoint: STPoint): VectorPoint {
  const { face: curFace, s, t, z, m } = stPoint;
  if (targetFace === curFace) return { x: s, y: t, z, m };

  const [rot, x, y] = FACE_RULE_SET[targetFace][curFace];
  const [newS, newT] = rotate(rot as 0 | 90 | -90, s, t);

  return { x: newS + x, y: newT + y, z, m };
}

/**
 * @param rot - rotation
 * @param s - input s
 * @param t - input t
 * @returns - new [s, t] after rotating
 */
function rotate(rot: 0 | 90 | -90, s: number, t: number): [s: number, t: number] {
  if (rot === 90) return [t, 1 - s];
  else if (rot === -90) return [1 - t, s];
  else return [s, t]; // Handles the 0° case and any other unspecified rotations
}

/**
 * Ruleset for converting an S2Point from a face to another.
 * While this this set includes opposite side faces, without axis mirroring,
 * it is not technically accurate and shouldn't be used. Instead, data should let two points travel
 * further than a full face width.
 * FACE_RULE_SET[targetFace][currentFace] = [rot, x, y]
 */
const FACE_RULE_SET: [rotation: number, moveX: number, MoveY: number][][] = [
  // Target Face 0
  [
    [0, 0, 0], // Current Face 0
    [0, 1, 0], // Current Face 1
    [90, 0, 1], // Current Face 2
    [-90, 2, 0], // Current Face 3
    [-90, -1, 0], ///  Current Face 4
    [0, 0, -1], ///  Current Face 5
  ],
  // Target Face 1
  [
    [0, -1, 0], // Current Face 0
    [0, 0, 0], // Current Face 1
    [0, 0, 1], // Current Face 2
    [-90, 1, 0], // Current Face 3
    [-90, 2, 0], // Current Face 4
    [90, 0, -1], // Current Face 5
  ],
  // Target Face 2
  [
    [-90, -1, 0], // Current Face 0
    [0, 0, -1], // Current Face 1
    [0, 0, 0], // Current Face 2
    [0, 1, 0], // Current Face 3
    [90, 0, 1], // Current Face 4
    [-90, 2, 0], // Current Face 5
  ],
  // Target Face 3
  [
    [-90, 2, 0], // Current Face 0
    [90, 0, -1], // Current Face 1
    [0, -1, 0], // Current Face 2
    [0, 0, 0], // Current Face 3
    [0, 0, 1], // Current Face 4
    [-90, 1, 0], // Current Face 5
  ],
  // Target Face 4
  [
    [90, 0, 1], // Current Face 0
    [-90, 2, 0], // Current Face 1
    [-90, -1, 0], // Current Face 2
    [0, 0, -1], // Current Face 3
    [0, 0, 0], // Current Face 4
    [0, 1, 0], // Current Face 5
  ],
  // Target Face 5
  [
    [0, 0, 1], // Current Face 0
    [-90, 1, 0], // Current Face 1
    [-90, 2, 0], // Current Face 2
    [90, 0, -1], // Current Face 3
    [0, -1, 0], // Current Face 4
    [0, 0, 0], // Current Face 5
  ],
];
