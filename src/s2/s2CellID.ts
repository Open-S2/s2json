/** COMPONENTS */
import {
  IJtoST,
  STtoIJ,
  quadraticSTtoUV as STtoUV,
  SiTiToST,
  quadraticUVtoST as UVtoST,
  XYZtoFaceUV,
  faceUVtoXYZ,
  lonLatToXYZ,
  xyzToLonLat,
} from './s2Coords';
import { toIJ as S2PointToIJ, fromS2CellID } from './s2Point';

import type { BBox, Face, Point3D } from '../';

/** CONSTANTS */
const LOOKUP_POS: bigint[] = [];
const LOOKUP_IJ: bigint[] = [];
const FACE_BITS = 3n;
const NUM_FACES = 6n;
const MAX_LEVEL = 30n;
const POS_BITS = 61n;
const K_WRAP_OFFSET = 13835058055282163712n;
const K_MAX_SIZE = 1073741824;

/** INITIALIZATION */
for (let i = 0; i < 4; i++) initLookupCell(0, 0, 0, i, 0, i);
/**
 * @param level - zoom level of the cell
 * @param i - x coord
 * @param j - y coord
 * @param origOrientation - original orientation
 * @param pos - position
 * @param orientation - orientation
 */
function initLookupCell(
  level: number,
  i: number,
  j: number,
  origOrientation: number,
  pos: number,
  orientation: number,
): void {
  const kPosToOriengation = [1, 0, 0, 3];
  const kPosToIJ = [
    [0, 1, 3, 2],
    [0, 2, 3, 1],
    [3, 2, 0, 1],
    [3, 1, 0, 2],
  ];
  if (level === 4) {
    const ij = (i << 4) + j;
    LOOKUP_POS[(ij << 2) + origOrientation] = BigInt((pos << 2) + orientation);
    LOOKUP_IJ[(pos << 2) + origOrientation] = BigInt((ij << 2) + orientation);
  } else {
    level++;
    i <<= 1;
    j <<= 1;
    pos <<= 2;
    const r = kPosToIJ[orientation];
    initLookupCell(
      level,
      i + (r[0] >> 1),
      j + (r[0] & 1),
      origOrientation,
      pos,
      orientation ^ kPosToOriengation[0],
    );
    initLookupCell(
      level,
      i + (r[1] >> 1),
      j + (r[1] & 1),
      origOrientation,
      pos + 1,
      orientation ^ kPosToOriengation[1],
    );
    initLookupCell(
      level,
      i + (r[2] >> 1),
      j + (r[2] & 1),
      origOrientation,
      pos + 2,
      orientation ^ kPosToOriengation[2],
    );
    initLookupCell(
      level,
      i + (r[3] >> 1),
      j + (r[3] & 1),
      origOrientation,
      pos + 3,
      orientation ^ kPosToOriengation[3],
    );
  }
}

/**
 * Create a default S2CellID given a face on the sphere [0-6)
 * @param face - the face
 * @returns the S2CellID
 */
export function fromFace(face: Face): bigint {
  return (BigInt(face) << POS_BITS) + (1n << 60n);
}

/**
 * Create an S2CellID from a lon-lat coordinate
 * @param lon - longitude
 * @param lat - latitude
 * @returns the S2CellID
 */
export function fromLonLat(lon: number, lat: number): bigint {
  const xyz = lonLatToXYZ(lon, lat);
  return fromS2Point(xyz);
}

/**
 * Create an S2CellID from an XYZ Point
 * @param xyz - 3D input vector
 * @returns the S2CellID
 */
export function fromS2Point(xyz: Point3D): bigint {
  // convert to face-i-j
  const [face, i, j] = S2PointToIJ(xyz);
  // now convert from ij
  return fromIJ(face, i, j);
}

/**
 * Create an S2CellID from an Face-U-V coordinate
 * @param face - the face
 * @param u - u coordinate
 * @param v - v coordinate
 * @returns the S2CellID
 */
export function fromUV(face: Face, u: number, v: number): bigint {
  // now convert from st
  return fromST(face, UVtoST(u), UVtoST(v));
}

/**
 * Create an S2CellID from an Face-S-T coordinate
 * @param face - the face
 * @param s - s coordinate
 * @param t - t coordinate
 * @returns the S2CellID
 */
export function fromST(face: Face, s: number, t: number): bigint {
  // now convert from ij
  return fromIJ(face, STtoIJ(s), STtoIJ(t));
}

/**
 * Create an S2CellID given a distance and level (zoom). Default level is 30n
 * @param distance - distance
 * @param level - level
 * @returns the S2CellID
 */
export function fromDistance(distance: bigint, level = MAX_LEVEL): bigint {
  level = 2n * (MAX_LEVEL - level);
  return (distance << (level + 1n)) + (1n << level);
}

/**
 * Create an S2CellID from an Face-I-J coordinate and map it to a zoom if desired.
 * @param face - the face
 * @param i - i coordinate
 * @param j - j coordinate
 * @param level - zoom level
 * @returns the S2CellID
 */
export function fromIJ(face: Face, i: number, j: number, level?: number): bigint {
  const bigFace = BigInt(face);
  let bigI = BigInt(i);
  let bigJ = BigInt(j);
  if (level !== undefined) {
    const levelB = BigInt(level);
    bigI = bigI << (MAX_LEVEL - levelB);
    bigJ = bigJ << (MAX_LEVEL - levelB);
  }
  let n = bigFace << 60n;
  // Alternating faces have opposite Hilbert curve orientations; this
  // is necessary in order for all faces to have a right-handed
  // coordinate system.
  let bits = bigFace & 1n;
  // Each iteration maps 4 bits of "i" and "j" into 8 bits of the Hilbert
  // curve position.  The lookup table transforms a 10-bit key of the form
  // "iiiijjjjoo" to a 10-bit value of the form "ppppppppoo", where the
  // letters [ijpo] denote bits of "i", "j", Hilbert curve position, and
  // Hilbert curve orientation respectively.
  for (let k = 7n; k >= 0n; k--) {
    const kk = k * 4n;
    bits += ((bigI >> kk) & 15n) << NUM_FACES;
    bits += ((bigJ >> kk) & 15n) << 2n;
    bits = LOOKUP_POS[Number(bits)];
    n |= (bits >> 2n) << (k * 8n);
    bits &= FACE_BITS;
  }

  const id = n * 2n + 1n;

  if (level !== undefined) return parent(id, level);
  return id;
}

/**
 * Convert an S2CellID to a Face-I-J coordinate and provide its orientation.
 * If a level is provided, the I-J coordinates will be shifted to that level.
 * @param id - the S2CellID
 * @param level - zoom level
 * @returns face-i-j with orientation
 */
export function toIJ(
  id: bigint,
  level?: number | bigint,
): [face: Face, i: number, j: number, orientation: number] {
  let i = 0n;
  let j = 0n;
  const face = Number(id >> POS_BITS);
  let bits = BigInt(face) & 1n;

  // Each iteration maps 8 bits of the Hilbert curve position into
  // 4 bits of "i" and "j".  The lookup table transforms a key of the
  // form "ppppppppoo" to a value of the form "iiiijjjjoo", where the
  // letters [ijpo] represents bits of "i", "j", the Hilbert curve
  // position, and the Hilbert curve orientation respectively.
  //
  // On the first iteration we need to be careful to clear out the bits
  // representing the cube face.
  for (let k = 7n; k >= 0n; k--) {
    const nbits = k === 7n ? 2n : 4n;
    bits += ((id >> (k * 8n + 1n)) & ((1n << (2n * nbits)) - 1n)) << 2n;
    bits = LOOKUP_IJ[Number(bits)];
    i += (bits >> NUM_FACES) << (k * 4n);
    j += ((bits >> 2n) & 15n) << (k * 4n);
    bits &= FACE_BITS;
  }

  // adjust bits to the orientation
  const lsb = id & (~id + 1n);
  if ((lsb & 1229782938247303424n) !== 0n) bits ^= 1n;

  if (level !== undefined) {
    level = BigInt(level);
    i = i >> (MAX_LEVEL - level);
    j = j >> (MAX_LEVEL - level);
  }
  return [face as Face, Number(i), Number(j), Number(bits)];
}

/**
 * Convert an S2CellID to an Face-S-T coordinate
 * @param id - the S2CellID
 * @returns face-s-t coordinate associated with the S2CellID
 */
export function toST(id: bigint): [face: Face, s: number, t: number] {
  const [face, i, j] = toIJ(id);
  const s = IJtoST(i);
  const t = IJtoST(j);

  return [face, s, t];
}

/**
 * Convert an S2CellID to an Face-U-V coordinate
 * @param id - the S2CellID
 * @returns face-u-v coordinate associated with the S2CellID
 */
export function toUV(id: bigint): [face: Face, u: number, v: number] {
  const [face, s, t] = toST(id);
  const u = STtoUV(s);
  const v = STtoUV(t);

  return [face, u, v];
}

/**
 * Convert an S2CellID to an lon-lat coordinate
 * @param id - the S2CellID
 * @returns lon-lat coordinates
 */
export function toLonLat(id: bigint): [lon: number, lat: number] {
  const xyz = toS2Point(id);

  return xyzToLonLat(xyz);
}

/**
 * Convert an S2CellID to an XYZ Point
 * @param id - the S2CellID
 * @returns a 3D vector
 */
export function toS2Point(id: bigint): Point3D {
  return fromS2CellID(id);
}

/**
 * Given an S2CellID, get the face it's located in
 * @param id - the S2CellID
 * @returns face of the cell
 */
export function face(id: bigint): Face {
  const face = Number(id >> POS_BITS);
  return face as Face;
}

/**
 * Given an S2CellID, check if it is a Face Cell.
 * @param id - the S2CellID
 * @returns true if the cell is a face (lowest zoom level)
 */
export function isFace(id: bigint): boolean {
  return (id & ((1n << 60n) - 1n)) === 0n;
}

/**
 * Given an S2CellID, find the quad tree position [0-4) it's located in
 * @param id - the S2CellID
 * @returns quad tree position
 */
export function pos(id: bigint): bigint {
  return id & 2305843009213693951n;
}

/**
 * Given an S2CellID, find the level (zoom) its located in
 * @param id - the S2CellID
 * @returns zoom level
 */
export function level(id: bigint): number {
  let count = 0;

  let i = 0n;
  while ((id & (1n << i)) === 0n && i < 60n) {
    i += 2n;
    count++;
  }

  return 30 - count;
}

/**
 * Given an S2CellID, get the distance it spans (or length it covers)
 * @param id - the S2CellID
 * @param lev - optional zoom level
 * @returns distance
 */
export function distance(id: bigint, lev?: number): bigint {
  if (lev === undefined) lev = level(id);
  return id >> BigInt(2 * (30 - lev) + 1);
}

/**
 * Given an S2CellID, get the quad child tile of your choice [0, 4)
 * @param id - the S2CellID
 * @param pos - quad position 0, 1, 2, or 3
 * @returns the child tile at that position
 */
export function child(id: bigint, pos: 0n | 1n | 2n | 3n): bigint {
  const newLSB = (id & (~id + 1n)) >> 2n;
  return id + (2n * pos - FACE_BITS) * newLSB;
}

/**
 * Given an S2CellID, get all the quad children tiles
 * @param id - the S2CellID
 * @param orientation - orientation of the child (0 or 1)
 * @returns the child tile at that position
 */
export function children(id: bigint, orientation = 0): [bigint, bigint, bigint, bigint] {
  const childs: [bigint, bigint, bigint, bigint] = [
    child(id, 0n),
    child(id, 3n),
    child(id, 2n),
    child(id, 1n),
  ];
  if (orientation === 0) {
    const tmp = childs[1];
    childs[1] = childs[3];
    childs[3] = tmp;
  }

  return childs;
}

/**
 * Given a Face-level-i-j coordinate, get all its quad children tiles
 * @param face - the Face
 * @param level - zoom level
 * @param i - i coordinate
 * @param j - j coordinate
 * @returns the child tile at that position
 */
export function childrenIJ(
  face: Face,
  level: number,
  i: number,
  j: number,
): [blID: bigint, brID: bigint, tlID: bigint, trID: bigint] {
  i = i << 1;
  j = j << 1;

  return [
    fromIJ(face, i, j, level + 1),
    fromIJ(face, i + 1, j, level + 1),
    fromIJ(face, i, j + 1, level + 1),
    fromIJ(face, i + 1, j + 1, level + 1),
  ];
}

/**
 * Given an S2CellID, get the quad position relative to its parent
 * @param id - the S2CellID
 * @param level - zoom level
 * @returns the child tile at that position
 */
export function childPosition(id: bigint, level: number): number {
  return Number((id >> (2n * (MAX_LEVEL - BigInt(level)) + 1n)) & FACE_BITS);
}

/**
 * Given an S2CellID, get the parent quad tile
 * @param id - the S2CellID
 * @param level - zoom level
 * @returns the parent of the input S2CellID
 */
export function parent(id: bigint, level?: number): bigint {
  const newLSB =
    level !== undefined ? 1n << (2n * (MAX_LEVEL - BigInt(level))) : (id & (~id + 1n)) << 2n;
  return (id & (~newLSB + 1n)) | newLSB;
}

/**
 * Given an S2CellID, get the hilbert range it spans
 * @param id - the S2CellID
 * @returns [min, max]
 */
export function range(id: bigint): [min: bigint, max: bigint] {
  const lsb = id & (~id + 1n);

  return [id - (lsb - 1n), id + (lsb - 1n)];
}

/**
 * Check if the first S2CellID contains the second.
 * @param a - the first S2CellID
 * @param b - the second S2CellID
 * @returns true if a contains b
 */
export function contains(a: bigint, b: bigint): boolean {
  const [min, max] = range(a);
  return b >= min && b <= max;
}

/**
 * Check if an S2CellID intersects another. This includes edges touching.
 * @param a - the first S2CellID
 * @param b - the second S2CellID
 * @returns true if a intersects b
 */
export function intersects(a: bigint, b: bigint): boolean {
  const [aMin, aMax] = range(a);
  const [bMin, bMax] = range(b);
  return bMin <= aMax && bMax >= aMin;
}

/**
 * Get the next S2CellID in the hilbert space
 * @param id - input S2CellID
 * @returns the next S2CellID in the hilbert space
 */
export function next(id: bigint): bigint {
  const n = id + ((id & (~id + 1n)) << 1n);
  if (n < K_WRAP_OFFSET) return n;
  return n - K_WRAP_OFFSET;
}

/**
 * Get the previous S2CellID in the hilbert space
 * @param id - input S2CellID
 * @returns the previous S2CellID in the hilbert space
 */
export function prev(id: bigint): bigint {
  const p = id - ((id & (~id + 1n)) << 1n);
  if (p < K_WRAP_OFFSET) return p;
  return p + K_WRAP_OFFSET;
}

/**
 * Check if the S2CellID is a leaf value. This means it's the smallest possible cell
 * @param id - input S2CellID
 * @returns true if the S2CellID is a leaf
 */
export function isLeaf(id: bigint): boolean {
  return (id & 1n) === 1n;
}

/**
 * Given an S2CellID and level (zoom), get the center point of that cell in S-T space
 * @param id - the S2CellID
 * @returns [face, s, t]
 */
export function centerST(id: bigint): [face: Face, s: number, t: number] {
  const [face, i, j] = toIJ(id);
  const delta = (id & 1n) !== 0n ? 1 : ((BigInt(i) ^ (id >> 2n)) & 1n) !== 0n ? 2 : 0;
  // Note that (2 * {i,j} + delta) will never overflow a 32-bit integer.
  const si = 2 * i + delta;
  const ti = 2 * j + delta;

  return [face, SiTiToST(Number(si)), SiTiToST(Number(ti))];
}

/**
 * Given an S2CellID and level (zoom), get the S-T bounding range of that cell
 * @param id - the S2CellID
 * @param lev - zoom level
 * @returns [sMin, tMin, sMax, tMax]
 */
export function boundsST(id: bigint, lev: number): BBox {
  if (lev === undefined) lev = level(id);

  const [, s, t] = centerST(id);
  const halfSize = sizeST(lev) * 0.5;

  return [s - halfSize, t - halfSize, s + halfSize, t + halfSize];
}

/**
 * Return the range maximum of a level (zoom) in S-T space
 * @param level - zoom level
 * @returns sMax or tMax
 */
export function sizeST(level: number): number {
  return IJtoST(sizeIJ(level));
}

/**
 * Return the range maximum of a level (zoom) in I-J space
 * @param level - zoom level
 * @returns iMax or jMax
 */
export function sizeIJ(level: number): number {
  return 1 << (30 - level);
}

/**
 * Given an S2CellID, find the neighboring S2CellIDs
 * @param id - the S2CellID
 * @returns [up, right, down, left]
 */
export function neighbors(id: bigint): [bigint, bigint, bigint, bigint] {
  const lev = level(id);
  const size = sizeIJ(lev);
  const [face, i, j] = toIJ(id);

  return [
    parent(fromIJSame(face, i, j - size, j - size >= 0), lev),
    parent(fromIJSame(face, i + size, j, i + size < K_MAX_SIZE), lev),
    parent(fromIJSame(face, i, j + size, j + size < K_MAX_SIZE), lev),
    parent(fromIJSame(face, i - size, j, i - size >= 0), lev),
  ];
}

/**
 * Given a Face-I-J and a desired level (zoom), find the neighboring S2CellIDs
 * @param face - the Face
 * @param i - the I coordinate
 * @param j - the J coordinate
 * @param level - the zoom level (desired)
 * @returns neighbors: [down, right, up, left]
 */
export function neighborsIJ(
  face: Face,
  i: number,
  j: number,
  level: number,
): [bigint, bigint, bigint, bigint] {
  const size = sizeIJ(level);

  return [
    parent(fromIJSame(face, i, j - size, j - size >= 0), level),
    parent(fromIJSame(face, i + size, j, i + size < K_MAX_SIZE), level),
    parent(fromIJSame(face, i, j + size, j + size < K_MAX_SIZE), level),
    parent(fromIJSame(face, i - size, j, i - size >= 0), level),
  ];
}

/**
 * Build an S2CellID given a Face-I-J, but ensure the face is the same if desired
 * @param face - the Face
 * @param i - the I coordinate
 * @param j - the J coordinate
 * @param sameFace - if the face should be the same
 * @returns the S2CellID
 */
export function fromIJSame(face: Face, i: number, j: number, sameFace: boolean): bigint {
  if (sameFace) return fromIJ(face, i, j);
  else return fromIJWrap(face, i, j);
}

/**
 * Build an S2CellID given a Face-I-J, but ensure it's a legal value, otherwise wrap before creation
 * @param face - the Face
 * @param i - the I coordinate
 * @param j - the J coordinate
 * @returns the S2CellID
 */
export function fromIJWrap(face: Face, i: number, j: number): bigint {
  const { max, min } = Math;

  // Convert i and j to the coordinates of a leaf cell just beyond the
  // boundary of this face.  This prevents 32-bit overflow in the case
  // of finding the neighbors of a face cell.
  i = max(-1, min(K_MAX_SIZE, i));
  j = max(-1, min(K_MAX_SIZE, j));

  // We want to wrap these coordinates onto the appropriate adjacent face.
  // The easiest way to do this is to convert the (i,j) coordinates to (x,y,z)
  // (which yields a point outside the normal face boundary), and then call
  // S2::XYZtoFaceUV() to project back onto the correct face.
  //
  // The code below converts (i,j) to (si,ti), and then (si,ti) to (u,v) using
  // the linear projection (u=2*s-1 and v=2*t-1).  (The code further below
  // converts back using the inverse projection, s=0.5*(u+1) and t=0.5*(v+1).
  // Any projection would work here, so we use the simplest.)  We also clamp
  // the (u,v) coordinates so that the point is barely outside the
  // [-1,1]x[-1,1] face rectangle, since otherwise the reprojection step
  // (which divides by the new z coordinate) might change the other
  // coordinates enough so that we end up in the wrong leaf cell.
  const kScale = 1 / K_MAX_SIZE;
  const kLimit = 1 + 2.2204460492503131e-16;
  const u = max(-kLimit, min(kLimit, kScale * (2 * (i - K_MAX_SIZE / 2) + 1)));
  const v = max(-kLimit, min(kLimit, kScale * (2 * (j - K_MAX_SIZE / 2) + 1)));

  // Find the leaf cell coordinates on the adjacent face, and convert
  // them to a cell id at the appropriate level.
  const [nFace, nU, nV] = XYZtoFaceUV(faceUVtoXYZ(face, u, v));
  return fromIJ(nFace, STtoIJ(0.5 * (nU + 1)), STtoIJ(0.5 * (nV + 1)));
}

/**
 * Given an S2CellID, find it's nearest neighbors associated with it
 * @param id - the S2CellID
 * @param lev - the zoom level (if not provided, defaults to current level of id)
 * @returns neighbors
 */
export function vertexNeighbors(id: bigint, lev?: number): bigint[] {
  if (lev === undefined) lev = level(id);
  const res: bigint[] = [];

  const [face, i, j] = toIJ(id);

  // Determine the i- and j-offsets to the closest neighboring cell in each
  // direction.  This involves looking at the next bit of "i" and "j" to
  // determine which quadrant of this->parent(level) this cell lies in.
  const halfsize = sizeIJ(lev + 1);
  const size = halfsize << 1;
  let isame: boolean, jsame: boolean, ioffset: number, joffset: number;

  if ((i & halfsize) !== 0) {
    ioffset = size;
    isame = i + size < K_MAX_SIZE;
  } else {
    ioffset = -size;
    isame = i - size >= 0;
  }
  if ((j & halfsize) !== 0) {
    joffset = size;
    jsame = j + size < K_MAX_SIZE;
  } else {
    joffset = -size;
    jsame = j - size >= 0;
  }

  res.push(parent(id, lev));
  res.push(parent(fromIJSame(face, i + ioffset, j, isame), lev));
  res.push(parent(fromIJSame(face, i, j + joffset, jsame), lev));
  if (isame || jsame)
    res.push(parent(fromIJSame(face, i + ioffset, j + joffset, isame && jsame), lev));

  return res;
}
