import {
  childrenIJ as childrenIJS2,
  contains as containsS2,
  face as faceS2,
  fromFace as fromFaceS2,
  isFace as isFaceS2,
  level as levelS2,
  parent as parentS2,
  toIJ as toIJS2,
} from './s2/s2CellID';
import {
  children as childrenWM,
  contains as containsWM,
  fromID as fromIDWM,
  isFace as isFaceWM,
  level as levelWM,
  parent as parentWM,
  toID as toIDWM,
  toIJ as toIJWM,
} from './wm/mercID';

import type { Face, Projection } from './';

/**
 * @param proj - the projection of choice
 * @param id - the tile ID
 * @returns - the tile broken into face-zoom-i-j
 */
export function fromID(
  proj: Projection,
  id: bigint,
): [face: Face, zoom: number, i: number, j: number] {
  if (proj === 'S2') {
    const zoom = levelS2(id);
    const [face, i, j] = toIJS2(id, zoom);
    return [face, zoom, i, j];
  } else return [0, ...fromIDWM(id)];
}

/**
 * Given the projection, get the parent tile of the input ID
 * @param proj - the projection
 * @param id - the input tile ID
 * @returns - the tile ID of the parent
 */
export function parent(proj: Projection, id: bigint): bigint {
  if (proj === 'S2') return parentS2(id);
  else return parentWM(id);
}

/**
 * Given the projection, get the children tiles given the face-zoom-i-j.
 * @param proj - the projection
 * @param face - the face
 * @param zoom - the zoom
 * @param i - x position in tile space
 * @param j - y position in tile space
 * @returns - the tile IDs of the children
 */
export function childrenIJ(
  proj: Projection,
  face: Face,
  zoom: number,
  i: number,
  j: number,
): [blID: bigint, brID: bigint, tlID: bigint, trID: bigint] {
  if (proj === 'S2') return childrenIJS2(face, zoom, i, j);
  else return childrenWM(toIDWM(zoom, i, j));
}

/**
 * Given the projection, check if the tile ID is of the highest zoom or not.
 * @param proj - the projection
 * @param id - the input tile ID
 * @returns - true if the tile ID is of the highest zoom
 */
export function isFace(proj: Projection, id: bigint): boolean {
  if (proj === 'S2') return isFaceS2(id);
  else return isFaceWM(id);
}

/**
 * Given the projection, get the face of the input ID. If not S2, gaurenteed to be 0.
 * @param proj - the projection
 * @param id - the input tile ID
 * @returns - the tile ID of the parent
 */
export function face(proj: Projection, id: bigint): Face {
  if (proj === 'S2') return faceS2(id);
  else return 0 as Face;
}

/**
 * Given the projection, get the ID of a requested face. If not S2, gaurenteed to be 0.
 * @param proj - the projection
 * @param face - the requested face
 * @returns - the ID of the requested face
 */
export function fromFace(proj: Projection, face: Face): bigint {
  if (proj === 'S2') return fromFaceS2(face);
  else return 0n;
}

/**
 * Given the projection and id, return the face/zoom with the i-j coordinate pair
 * @param proj - the projection
 * @param id - the input tile ID
 * @param level - the requested zoom
 * @returns - the ID of the requested zoom
 */
export function toIJ(
  proj: Projection,
  id: bigint,
  level?: number | bigint,
): [faceOrZoom: number, i: number, j: number] {
  if (proj === 'S2') return toIJS2(id, level).slice(0, 3) as [face: number, i: number, j: number];
  else return toIJWM(id, level);
}

/**
 * Given the projection and tile ID, get the zoom of said tile.
 * @param proj - the projection
 * @param id - the input tile ID
 * @returns - the zoom of the tile
 */
export function level(proj: Projection, id: bigint): number {
  if (proj === 'S2') return levelS2(id);
  else return levelWM(id);
}

/**
 * Given the projection, check if the "child" exists inside the "parent"'s bounds or not.
 * @param proj - the projection
 * @param parentID - the parent
 * @param childID - the child
 * @returns - true if the "child" exists inside the "parent"'s bounds
 */
export function contains(proj: Projection, parentID: bigint, childID: bigint): boolean {
  if (proj === 'S2') return containsS2(parentID, childID);
  else return containsWM(parentID, childID);
}