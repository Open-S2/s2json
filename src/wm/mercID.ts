/**
 * Convert zoom-x-y to a singular number
 * @param zoom - zoom level
 * @param x - x coord
 * @param y - y coord
 * @returns - The ID associated with the tile
 */
export function toID(zoom: number, x: number, y: number): bigint {
  if (zoom < 0 || zoom > 30) return BigInt(-1);

  // Encode the zoom level into the first 6 bits
  const zoomPart = BigInt(zoom) << 58n;
  // Encode the x coordinate into the next 29 bits
  const xPart = (BigInt(x) & ((1n << 29n) - 1n)) << 29n;
  // Encode the y coordinate into the remaining 29 bits
  const yPart = BigInt(y) & ((1n << 29n) - 1n);

  // Combine all parts into a single u64
  return zoomPart | xPart | yPart;
}

/**
 * Convert a number or bigint to [zoom, x, y]
 * @param idB - the tile ID
 * @returns - tile coordinates
 */
export function fromID(idB: bigint): [zoom: number, x: number, y: number] {
  const zoom = Number(idB >> 58n);
  const x = Number((idB >> 29n) & ((1n << 29n) - 1n));
  const y = Number(idB & ((1n << 29n) - 1n));

  return [zoom, x, y];
}

/**
 * Given a tile ID, find the 4 children tile IDs
 * @param id - the tile ID
 * @returns - 4 tile IDs of the children associated with the tile
 */
export function children(id: bigint): [blID: bigint, brID: bigint, tlID: bigint, trID: bigint] {
  const [zoom, x, y] = fromID(id);
  const childZoom = zoom + 1;
  const childX = x * 2;
  const childY = y * 2;
  return [
    toID(childZoom, childX, childY),
    toID(childZoom, childX + 1, childY),
    toID(childZoom, childX, childY + 1),
    toID(childZoom, childX + 1, childY + 1),
  ];
}

/**
 * grab the tiles next to the current tiles zoom-x-y
 * only include adjacent tiles, not diagonal.
 * If includeOutOfBounds set to true, it will include out of bounds tiles
 * on the x-axis
 * @param zoom - zoom level
 * @param x - x coord
 * @param y - y coord
 * @param includeOutOfBounds - if true, it will include out of bounds tiles on the x-axis
 * @returns - list of adjacent tiles
 */
export function neighborsXY(
  zoom: number,
  x: number,
  y: number,
  includeOutOfBounds = false,
): [zoom: number, x: number, y: number][] {
  const size = 1 << zoom;
  const neighbors: [zoom: number, x: number, y: number][] = [];
  const xOutOfBounds = x < 0 || x >= size;
  if (x - 1 >= 0 || includeOutOfBounds) neighbors.push([zoom, x - 1, y]);
  if (x + 1 < size || includeOutOfBounds) neighbors.push([zoom, x + 1, y]);
  if (!xOutOfBounds && y - 1 >= 0) neighbors.push([zoom, x, y - 1]);
  if (!xOutOfBounds && y + 1 < size) neighbors.push([zoom, x, y + 1]);
  return neighbors;
}

/**
 * Check if the tile is not a real world tile that fits inside the quad tree
 * Out of bounds tiles exist if the map has `duplicateHorizontally` set to true.
 * This is useful for filling in the canvas on the x axis instead of leaving it blank.
 * @param id - the tile ID
 * @returns - true if the tile is out of bounds
 */
export function isOutOfBounds(id: bigint): boolean {
  const [zoom, x, y] = fromID(id);
  const size = 1 << zoom;
  return x < 0 || y < 0 || x >= size || y >= size;
}

/**
 * Given a tile ID, find the "wrapped" tile ID.
 * It may resolve to itself. This is useful for maps that have
 * `duplicateHorizontally` set to true. It forces the tile to be
 * within the bounds of the quad tree.
 * @param id - the tile ID
 * @returns - the wrapped tile ID
 */
export function tileIDWrapped(id: bigint): bigint {
  const [zoom, x, y] = fromID(id);
  const size = 1 << zoom;
  return toID(zoom, x % size, y % size);
}

/**
 * Given a tileID, find the parent tile
 * @param id - the tile ID
 * @returns - the parent tile
 */
export function parent(id: bigint): bigint {
  const [z, x, y] = fromID(id);
  return toID(z - 1, Math.floor(x / 2), Math.floor(y / 2));
}

/**
 * convert an id to a zoom-x-y after setting it to a new parent zoom
 * @param id - the tile ID
 * @param level - the new zoom level (if not set, it will be the same as the old zoom level)
 * @returns - the new zoom-x-y
 */
export function toIJ(id: bigint, level?: number | bigint): [zoom: number, x: number, y: number] {
  if (level !== undefined) {
    let [currentZoom] = fromID(id);
    while (level < currentZoom) {
      id = parent(id);
      currentZoom--;
    }
  }
  return fromID(id);
}

/**
 * Check if the parentID contains the childID within the sub quads
 * @param parentID - the parent ID
 * @param childID - the child ID
 * @returns - true if the parent contains the child
 */
export function contains(parentID: bigint, childID: bigint): boolean {
  const [pz, px, py] = fromID(parentID);
  const [cz, cx, cy] = fromID(childID);
  if (pz > cz) return false;
  // Calculate the difference of child at the parent's level
  const diff = cz - pz;
  // check if x and y match adjusting child x,y to parent's level
  return px === cx >> diff && py === cy >> diff;
}

/**
 * Given a Tile ID, check if the zoom is 0 or not
 * @param id - the tile ID
 * @returns - true if the zoom is 0
 */
export function isFace(id: bigint): boolean {
  return fromID(id)[0] === 0;
}

/**
 * Get the zoom from the tile ID
 * @param id - the tile ID
 * @returns - the zoom
 */
export function level(id: bigint): number {
  return fromID(id)[0];
}
