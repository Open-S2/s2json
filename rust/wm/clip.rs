// import { childrenIJ, fromID } from '../id';
// import { clipBBox, extendBBox } from '../bbox';

// import type {
//   BBOX,
//   BBox,
//   MValue,
//   Projection,
//   VectorFeature,
//   VectorGeometry,
//   VectorLineString,
//   VectorLineStringGeometry,
//   VectorMultiLineOffset,
//   VectorMultiLineStringGeometry,
//   VectorMultiPointGeometry,
//   VectorMultiPolygonGeometry,
//   VectorPoint,
//   VectorPointGeometry,
//   VectorPolygon,
//   VectorPolygonGeometry,
// } from '../';

// /** Split faetures into the 4 children of a tile */
// export type TileChildren = [
//   { id: bigint; features: VectorFeature[] }, // bottom left
//   { id: bigint; features: VectorFeature[] }, // bottom right
//   { id: bigint; features: VectorFeature[] }, // top left
//   { id: bigint; features: VectorFeature[] }, // top right
// ];

// /**
//  * @param projection - WM or S2
//  * @param id - tile id that is either WM or S2
//  * @param features - Vector Features
//  * @param buffer - optional buffer to apply to the tile
//  * @returns - the tile's children split into 4 sub-tiles
//  */
// export function splitTile(
//   projection: Projection,
//   id: bigint,
//   features: VectorFeature[],
//   buffer: number = 0.0625,
// ): TileChildren {
//   const [face, zoom, i, j] = fromID(projection, id);
//   const [blID, brID, tlID, trID] = childrenIJ(projection, face, zoom, i, j);
//   const children: TileChildren = [
//     { id: blID, features: [] },
//     { id: brID, features: [] },
//     { id: tlID, features: [] },
//     { id: trID, features: [] },
//   ];
//   const scale = 1 << zoom;
//   const k1 = 0;
//   const k2 = 0.5;
//   const k3 = 0.5;
//   const k4 = 1;

//   let tl: null | VectorFeature[] = null;
//   let bl: null | VectorFeature[] = null;
//   let tr: null | VectorFeature[] = null;
//   let br: null | VectorFeature[] = null;

//   const left = _clip(features, scale, i - k1, i + k3, 0, buffer);
//   const right = _clip(features, scale, i + k2, i + k4, 0, buffer);

//   if (left !== null) {
//     bl = _clip(left, scale, j - k1, j + k3, 1, buffer);
//     tl = _clip(left, scale, j + k2, j + k4, 1, buffer);
//     if (bl !== null) for (const d of bl) children[0].features.push(d);
//     if (tl !== null) for (const d of tl) children[2].features.push(d);
//   }

//   if (right !== null) {
//     br = _clip(right, scale, j - k1, j + k3, 1, buffer);
//     tr = _clip(right, scale, j + k2, j + k4, 1, buffer);
//     if (br !== null) for (const d of br) children[1].features.push(d);
//     if (tr !== null) for (const d of tr) children[3].features.push(d);
//   }

//   return children;
// }

// /**
//  * @param features - input features to clip
//  * @param scale - the tile scale
//  * @param k1 - minimum accepted value of the axis
//  * @param k2 - maximum accepted value of the axis
//  * @param axis - the axis 0 for x, 1 for y
//  * @param baseBuffer - the top level buffer value
//  * @returns - the clipped features
//  */
// function _clip(
//   features: VectorFeature[],
//   scale: number,
//   k1: number,
//   k2: number,
//   axis: 0 | 1,
//   baseBuffer: number,
// ): null | VectorFeature[] {
//   // scale
//   k1 /= scale;
//   k2 /= scale;
//   // prep buffer and result container
//   const buffer = baseBuffer / scale;
//   const k1b = k1 - buffer;
//   const k2b = k2 + buffer;
//   const clipped: VectorFeature[] = [];
//   const axis_x = axis === 0;

//   for (const feature of features) {
//     const { geometry } = feature;
//     const { type, vecBBox } = geometry;
//     // trivial accept and reject cases
//     if (vecBBox !== undefined) {
//       const min = axis_x ? vecBBox[0] : vecBBox[1];
//       const max = axis_x ? vecBBox[2] : vecBBox[3];
//       if (min >= k1 && max < k2) {
//         clipped.push(feature);
//         continue;
//       } else if (max < k1 || min >= k2) {
//         continue;
//       }
//     }
//     // build the new clipped geometry
//     let newGeometry: VectorGeometry | undefined = undefined;
//     if (type === 'Point') newGeometry = clipPoint(geometry, axis, k1, k2);
//     else if (type === 'MultiPoint') newGeometry = clipMultiPoint(geometry, axis, k1, k2);
//     else if (type === 'LineString') newGeometry = clipLineString(geometry, axis, k1b, k2b);
//     else if (type === 'MultiLineString')
//       newGeometry = clipMultiLineString(geometry, axis, k1b, k2b);
//     else if (type === 'Polygon') newGeometry = clipPolygon(geometry, axis, k1b, k2b);
//     else if (type === 'MultiPolygon') newGeometry = clipMultiPolygon(geometry, axis, k1b, k2b);
//     // store if the geometry was inside the range
//     if (newGeometry !== undefined) {
//       newGeometry.vecBBox = clipBBox(newGeometry.vecBBox, axis, k1b, k2b);
//       clipped.push({ ...feature, geometry: newGeometry });
//     }
//   }

//   return clipped.length > 0 ? clipped : null;
// }

// /**
//  * @param geometry - input vector geometry
//  * @param axis - 0 for x, 1 for y
//  * @param k1 - minimum accepted value of the axis
//  * @param k2 - maximum accepted value of the axis
//  * @returns - the clipped geometry or undefined if the geometry was not inside the range
//  */
// function clipPoint(
//   geometry: VectorPointGeometry,
//   axis: 0 | 1,
//   k1: number,
//   k2: number,
// ): VectorPointGeometry | undefined {
//   const { type, coordinates, bbox, vecBBox } = geometry;
//   const value = axis === 0 ? coordinates.x : coordinates.y;
//   if (value >= k1 && value < k2) return { type, coordinates, bbox, vecBBox };
// }

// /**
//  * @param geometry - input vector geometry
//  * @param axis - 0 for x, 1 for y
//  * @param k1 - minimum accepted value of the axis
//  * @param k2 - maximum accepted value of the axis
//  * @returns - the clipped geometry or undefined if the geometry was not inside the range
//  */
// function clipMultiPoint(
//   geometry: VectorMultiPointGeometry,
//   axis: 0 | 1,
//   k1: number,
//   k2: number,
// ): VectorMultiPointGeometry | undefined {
//   const { type, coordinates, bbox } = geometry;
//   let vecBBox: BBOX | undefined = undefined;
//   const points = coordinates.filter((point) => {
//     const value = axis === 0 ? point.x : point.y;
//     return value >= k1 && value < k2;
//   });
//   points.forEach((p) => (vecBBox = extendBBox(vecBBox, p)));

//   if (points.length > 0) return { type, coordinates: points, bbox, vecBBox };
// }

// /**
//  * @param geometry - input vector geometry
//  * @param axis - 0 for x, 1 for y
//  * @param k1 - minimum accepted value of the axis
//  * @param k2 - maximum accepted value of the axis
//  * @returns - the clipped geometry or undefined if the geometry was not inside the range
//  */
// function clipLineString(
//   geometry: VectorLineStringGeometry,
//   axis: 0 | 1,
//   k1: number,
//   k2: number,
// ): VectorMultiLineStringGeometry | undefined {
//   const { coordinates: line, bbox, vecBBox } = geometry;
//   const initO = geometry.offset ?? 0;
//   const newOffsets: VectorMultiLineOffset = [];
//   const newLines: VectorLineString[] = [];
//   for (const clip of _clipLine({ line, offset: initO }, k1, k2, axis, false)) {
//     newOffsets.push(clip.offset);
//     newLines.push(clip.line);
//   }
//   if (newLines.length === 0) return undefined;
//   return { type: 'MultiLineString', coordinates: newLines, bbox, offset: newOffsets, vecBBox };
// }

// /**
//  * @param geometry - input vector geometry
//  * @param axis - 0 for x, 1 for y
//  * @param k1 - minimum accepted value of the axis
//  * @param k2 - maximum accepted value of the axis
//  * @param isPolygon - true if the geometry is a polygon
//  * @returns - the clipped geometry or undefined if the geometry was not inside the range
//  */
// function clipMultiLineString(
//   geometry: VectorMultiLineStringGeometry | VectorPolygonGeometry,
//   axis: 0 | 1,
//   k1: number,
//   k2: number,
//   isPolygon = false,
// ): VectorMultiLineStringGeometry | undefined {
//   const { coordinates, bbox, vecBBox } = geometry;
//   const initO = geometry.offset ?? [];
//   const newOffsets: VectorMultiLineOffset = [];
//   const newLines: VectorLineString[] = [];
//   coordinates.forEach((line, i) => {
//     for (const clip of _clipLine({ line, offset: initO[i] }, k1, k2, axis, isPolygon)) {
//       newOffsets.push(clip.offset);
//       newLines.push(clip.line);
//     }
//   });
//   if (newLines.length === 0) return undefined;
//   return { type: 'MultiLineString', coordinates: newLines, bbox, offset: newOffsets, vecBBox };
// }

// /**
//  * @param geometry - input vector geometry
//  * @param axis - 0 for x, 1 for y
//  * @param k1 - minimum accepted value of the axis
//  * @param k2 - maximum accepted value of the axis
//  * @returns - the clipped geometry or undefined if the geometry was not inside the range
//  */
// function clipPolygon(
//   geometry: VectorPolygonGeometry,
//   axis: 0 | 1,
//   k1: number,
//   k2: number,
// ): VectorPolygonGeometry | undefined {
//   const multiline = clipMultiLineString(geometry, axis, k1, k2, true);
//   // @ts-expect-error - we are updating the type to a Polygon
//   multiline.type = 'Polygon';
//   if (
//     multiline === undefined ||
//     multiline.coordinates.length === 0 ||
//     multiline.coordinates[0].length < 4
//   )
//     return undefined;
//   return multiline as unknown as VectorPolygonGeometry;
// }

// /**
//  * @param geometry - input vector geometry
//  * @param axis - 0 for x, 1 for y
//  * @param k1 - minimum accepted value of the axis
//  * @param k2 - maximum accepted value of the axis
//  * @returns - the clipped geometry or undefined if the geometry was not inside the range
//  */
// function clipMultiPolygon(
//   geometry: VectorMultiPolygonGeometry,
//   axis: 0 | 1,
//   k1: number,
//   k2: number,
// ): VectorMultiPolygonGeometry | undefined {
//   const { coordinates, bbox, vecBBox } = geometry;
//   const newCoordinates: VectorPolygon[] = [];
//   coordinates.forEach((polygon) => {
//     const newPolygon = clipPolygon({ type: 'Polygon', coordinates: polygon, bbox }, axis, k1, k2);
//     if (newPolygon !== undefined) newCoordinates.push(newPolygon.coordinates);
//   });
//   if (newCoordinates.length === 0) return undefined;
//   return { type: 'MultiPolygon', coordinates: newCoordinates, bbox, vecBBox };
// }

// /**
//  * After clipping a line, return the altered line,
//  * the offset the new line starts at,
//  * and if the line is ccw
//  */
// export interface ClipLineResult {
//   line: VectorLineString;
//   offset: number;
//   vecBBox?: BBOX;
// }
// /** Ensuring `vecBBox` exists */
// export interface ClipLineResultWithBBox {
//   line: VectorLineString;
//   offset: number;
//   vecBBox: BBOX;
// }

// /**
//  * Data should always be in a 0->1 coordinate system to use this clip function
//  * @param geom - the original geometry line
//  * @param bbox - the bounding box to clip the line to
//  * @param isPolygon - true if the line comes from a polygon
//  * @param offset - the starting offset the line starts at
//  * @param buffer - the buffer to apply to the line (spacing outside the bounding box)
//  * @returns - the clipped geometry
//  */
// export function clipLine(
//   geom: VectorLineString,
//   bbox: BBox,
//   isPolygon: boolean,
//   offset: number = 0,
//   buffer: number = 0.0625, // default for a full size tile. Assuming 1024 extent and a 64 point buffer
// ): ClipLineResultWithBBox[] {
//   const res: ClipLineResult[] = [];
//   const [left, bottom, right, top] = bbox;
//   // clip horizontally
//   const horizontalClips = _clipLine(
//     { line: geom, offset, vecBBox: [0, 0, 0, 0] },
//     left - buffer,
//     right + buffer,
//     0,
//     isPolygon,
//   );
//   for (const clip of horizontalClips) {
//     // clip vertically
//     res.push(..._clipLine(clip, bottom - buffer, top + buffer, 1, isPolygon));
//   }
//   return res.map((clip) => {
//     let vecBBox: BBOX | undefined;
//     for (const p of clip.line) vecBBox = extendBBox(vecBBox, p);
//     clip.vecBBox = vecBBox;
//     return clip;
//   }) as ClipLineResultWithBBox[];
// }

// /**
//  * @param input - the original geometry line
//  * @param k1 - the lower bound
//  * @param k2 - the upper bound
//  * @param axis - 0 for x, 1 for y
//  * @param isPolygon - true if the line comes from a polygon
//  * @returns - the clipped geometry
//  */
// function _clipLine(
//   input: ClipLineResult,
//   k1: number,
//   k2: number,
//   axis: 0 | 1,
//   isPolygon: boolean,
// ): ClipLineResult[] {
//   const { line: geom, offset: startOffset } = input;
//   const newGeom: ClipLineResult[] = [];
//   let slice: VectorLineString = [];
//   let last = geom.length - 1;
//   const intersect = axis === 0 ? intersectX : intersectY;

//   let curOffset = startOffset;
//   let accOffset = startOffset;
//   let prevP = geom[0];
//   let firstEnter = false;

//   for (let i = 0; i < last; i++) {
//     const { x: ax, y: ay, z: az, m: am } = geom[i];
//     const { x: bx, y: by, z: bz, m: bm } = geom[i + 1];
//     const a = axis === 0 ? ax : ay;
//     const b = axis === 0 ? bx : by;
//     const z = az && bz ? (az + bz) / 2 : az ? az : bz ? bz : undefined;
//     let entered = false;
//     let exited = false;
//     let intP: VectorPoint | undefined;

//     // ENTER OR CONTINUE CASES
//     if (a < k1) {
//       // ---|-->  | (line enters the clip region from the left)
//       if (b > k1) {
//         intP = intersect(ax, ay, bx, by, k1, z, bm);
//         slice.push(intP);
//         entered = true;
//       }
//     } else if (a > k2) {
//       // |  <--|--- (line enters the clip region from the right)
//       if (b < k2) {
//         intP = intersect(ax, ay, bx, by, k2, z, bm);
//         slice.push(intP);
//         entered = true;
//       }
//     } else {
//       intP = { x: ax, y: ay, z: az, m: am };
//       slice.push(intP);
//     }

//     // Update the intersection point and offset if the intP exists
//     if (intP) {
//       // our first enter will change the offset for the line
//       if (entered && !firstEnter) {
//         curOffset = accOffset + distance(prevP, intP);
//         firstEnter = true;
//       }
//     }

//     // EXIT CASES
//     if (b < k1 && a >= k1) {
//       // <--|---  | or <--|-----|--- (line exits the clip region on the left)
//       intP = intersect(ax, ay, bx, by, k1, z, bm ?? am);
//       slice.push(intP);
//       exited = true;
//     }
//     if (b > k2 && a <= k2) {
//       // |  ---|--> or ---|-----|--> (line exits the clip region on the right)
//       intP = intersect(ax, ay, bx, by, k2, z, bm ?? am);
//       slice.push(intP);
//       exited = true;
//     }

//     // update the offset
//     accOffset += distance(prevP, geom[i + 1]);
//     prevP = geom[i + 1];

//     // If not a polygon, we can cut it into parts, otherwise we just keep tracking the edges
//     if (!isPolygon && exited) {
//       newGeom.push({ line: slice, offset: curOffset });
//       slice = [];
//       firstEnter = false;
//     }
//   }

//   // add the last point if inside the clip
//   const lastPoint = geom[last];
//   const a = axis === 0 ? lastPoint.x : lastPoint.y;
//   if (a >= k1 && a <= k2) slice.push({ ...lastPoint });

//   // close the polygon if its endpoints are not the same after clipping
//   if (slice.length > 0 && isPolygon) {
//     last = slice.length - 1;
//     const firstP = slice[0];
//     if (last >= 1 && (slice[last].x !== firstP.x || slice[last].y !== firstP.y)) {
//       slice.push({ ...firstP });
//     }
//   }

//   // add the final slice
//   if (slice.length > 0) newGeom.push({ line: slice, offset: curOffset });

//   return newGeom;
// }

// /**
//  * @param ax - the first x
//  * @param ay - the first y
//  * @param bx - the second x
//  * @param by - the second y
//  * @param x - the x to intersect
//  * @param z - the elevation if it exists
//  * @param m - the MValue
//  * @returns - the intersecting point
//  */
// function intersectX(
//   ax: number,
//   ay: number,
//   bx: number,
//   by: number,
//   x: number,
//   z?: number,
//   m?: MValue,
// ): VectorPoint {
//   const t = (x - ax) / (bx - ax);
//   return { x, y: ay + (by - ay) * t, z, m };
// }

// /**
//  * @param ax - the first x
//  * @param ay - the first y
//  * @param bx - the second x
//  * @param by - the second y
//  * @param y - the y to intersect
//  * @param z - the elevation if it exists
//  * @param m - the MValue
//  * @returns - the intersecting point
//  */
// function intersectY(
//   ax: number,
//   ay: number,
//   bx: number,
//   by: number,
//   y: number,
//   z?: number,
//   m?: MValue,
// ): VectorPoint {
//   const t = (y - ay) / (by - ay);
//   return { x: ax + (bx - ax) * t, y, z, m };
// }

// /**
//  * Calculate the Euclidean distance between two points.
//  * @param p1 - The first point.
//  * @param p2 - The second point.
//  * @returns - The distance between the points.
//  */
// function distance(p1: VectorPoint, p2: VectorPoint): number {
//   const { sqrt, pow } = Math;
//   return sqrt(pow(p2.x - p1.x, 2) + pow(p2.y - p1.y, 2));
// }