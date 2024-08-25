import type { BBox, Point } from './';

/**
 * Checks if a point is within a bounding box
 * @param bbox - the bounding box to test
 * @param point - point to test if it exists within the bbox
 * @returns - true if the point is within the bbox, false otherwise
 */
export function pointOverlap(bbox: BBox, point: Point): boolean {
  const [left, bottom, right, top] = bbox;
  return point[0] >= left && point[0] <= right && point[1] >= bottom && point[1] <= top;
}

/**
 * Checks if two bounding boxes overlap. If they don't overlap, returns undefined.
 * If they do, return the overlap
 * @param b1 - first bounding box
 * @param b2 - second bounding box
 * @returns - undefined if no overlap, or a bbox of the overlap
 */
export function bboxOverlap(b1: BBox, b2: BBox): undefined | BBox {
  // check if the bboxes overlap at all
  if (b2[2] < b1[0] || b1[2] < b2[0] || b2[3] < b1[1] || b1[3] < b2[1]) return;
  // find the middle two X values
  const left = b1[0] < b2[0] ? b2[0] : b1[0];
  const right = b1[2] < b2[2] ? b1[2] : b2[2];
  // find the middle two Y values
  const bottom = b1[1] < b2[1] ? b2[1] : b1[1];
  const top = b1[3] < b2[3] ? b1[3] : b2[3];

  return [left, bottom, right, top];
}
