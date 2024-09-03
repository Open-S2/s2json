import { convert } from './convert';
import { simplify } from '.';
import { splitTile } from './clip';
import {
  contains,
  fromFace,
  fromID,
  face as getFace,
  isFace,
  level,
  parent as parentID,
} from './id';

import type {
  Face,
  JSONCollection,
  Projection,
  VectorFeatures,
  VectorGeometry,
  VectorPoint,
} from '.';

/** Tile Class to contain the tile information for splitting or simplifying */
export class Tile {
  /**
   * @param id - the tile id
   * @param projection - WM or S2
   * @param layers - the tile's layers
   * @param transformed - whether the tile feature geometry has been transformed to tile coordinates
   */
  constructor(
    public id: bigint,
    public projection: Projection,
    public layers: Record<string, Layer> = {},
    public transformed = false,
  ) {}

  /** @returns true if the tile is empty of features */
  isEmpty(): boolean {
    for (const layer of Object.values(this.layers)) {
      if (layer.features.length > 0) return false;
    }
    return true;
  }

  /**
   * @param feature - Vector Feature
   * @param layer - layer to store the feature to
   */
  addFeature(feature: VectorFeatures, layer?: string): void {
    const { metadata = {} } = feature;

    const layerName = (metadata.layer as string) ?? layer ?? 'default';
    if (!this.layers[layerName]) {
      this.layers[layerName] = new Layer(layerName, []);
    }
    this.layers[layerName].features.push(feature);
  }

  /**
   * Simplify the geometry to have a tolerance which will be relative to the tile's zoom level.
   * NOTE: This should be called after the tile has been split into children if that functionality
   * is needed.
   * @param tolerance - tolerance
   * @param maxzoom - max zoom at which to simplify
   */
  transform(tolerance: number, maxzoom?: number) {
    if (this.transformed) return;
    const [, zoom, i, j] = fromID(this.projection, this.id);

    for (const layer of Object.values(this.layers)) {
      for (const feature of layer.features) {
        simplify(feature.geometry, tolerance, zoom, maxzoom);
        _transform(feature.geometry, zoom, i, j);
      }
    }

    this.transformed = true;
  }
}

/**
 * @param geometry - input vector geometry to be mutated in place
 * @param zoom - tile zoom
 * @param ti - tile i
 * @param tj - tile j
 */
function _transform(geometry: VectorGeometry, zoom: number, ti: number, tj: number) {
  const { type, coordinates } = geometry;
  zoom = 1 << zoom;

  if (type === 'Point') transformPoint(coordinates, zoom, ti, tj);
  else if (type === 'MultiPoint' || type === 'LineString')
    coordinates.forEach((p) => transformPoint(p, zoom, ti, tj));
  else if (type === 'MultiLineString' || type === 'Polygon')
    coordinates.forEach((l) => l.forEach((p) => transformPoint(p, zoom, ti, tj)));
  else if (type === 'MultiPolygon')
    coordinates.forEach((p) => p.forEach((l) => l.forEach((p) => transformPoint(p, zoom, ti, tj))));
}

/**
 * @param vp - input vector point that we are mutating in place
 * @param zoom - current zoom
 * @param ti - x translation
 * @param tj - y translation
 */
export function transformPoint(vp: VectorPoint, zoom: number, ti: number, tj: number) {
  vp.x = vp.x * zoom - ti;
  vp.y = vp.y * zoom - tj;
}

/** Layer Class to contain the layer information for splitting or simplifying */
export class Layer {
  /**
   * @param name - the layer name
   * @param features - the layer's features
   */
  constructor(
    public name: string,
    public features: VectorFeatures[] = [],
  ) {}
}

/** Options for creating a TileStore */
export interface TileStoreOptions {
  /** manually set the projection, otherwise it defaults to whatever the data type is */
  projection?: Projection;
  /** min zoom to generate data on */
  minzoom?: number;
  /** max zoom level to cluster the points on */
  maxzoom?: number;
  /** tile buffer on each side in pixels */
  indexMaxzoom?: number;
  /** simplification tolerance (higher means simpler) */
  tolerance?: number;
  /** tile buffer on each side so lines and polygons don't get clipped */
  buffer?: number;
}

/** TileStore Class is a tile-lookup system that splits and simplifies as needed for each tile request */
export class TileStore {
  minzoom = 0; // min zoom to preserve detail on
  maxzoom = 18; // max zoom to preserve detail on
  faces = new Set<Face>(); // store which faces are active. 0 face could be entire WM projection
  indexMaxzoom = 4; // max zoom in the tile index
  tolerance = 3; // simplification tolerance (higher means simpler)
  buffer = 0.0625; // tile buffer for lines and polygons
  tiles: Map<bigint, Tile> = new Map(); // stores both WM and S2 tiles
  projection: Projection; // projection to build tiles for
  /**
   * @param data - input data may be WM or S2 as a Feature or a Collection of Features
   * @param options - options to define how to store the data
   */
  constructor(data: JSONCollection, options?: TileStoreOptions) {
    // set options should they exist
    this.minzoom = options?.minzoom ?? 0;
    this.maxzoom = options?.maxzoom ?? 20;
    this.indexMaxzoom = options?.indexMaxzoom ?? 4;
    this.tolerance = options?.tolerance ?? 3;
    this.buffer = options?.buffer ?? 64;
    // update projection
    if (options?.projection !== undefined) this.projection = options.projection;
    else if (data.type === 'Feature' || data.type === 'FeatureCollection') this.projection = 'WM';
    else this.projection = 'S2';
    // sanity check
    if (this.maxzoom < 0 || this.maxzoom > 20)
      throw new Error('maxzoom should be in the 0-20 range');
    // convert features
    const features: VectorFeatures[] = convert(this.projection, data);
    for (const feature of features) this.addFeature(feature);
    for (let face = 0; face < 6; face++) {
      const id = fromFace(this.projection, face as Face);
      this.splitTile(id);
    }
  }

  /**
   * @param feature - the feature to store to a face tile. Creates the tile if it doesn't exist
   */
  addFeature(feature: VectorFeatures): void {
    const { faces, tiles, projection } = this;
    const face = 'face' in feature ? feature.face : 0;
    const id = fromFace(projection, face);
    let tile = tiles.get(id);
    if (tile === undefined) {
      faces.add(face);
      tile = new Tile(id, projection);
      tiles.set(id, tile);
    }
    tile?.addFeature(feature);
  }

  /**
   * @param startID - where to start tiling
   * @param endID - where to stop tiling
   * @param endZoom - stop tiling at this zoom
   */
  splitTile(startID: bigint, endID?: bigint, endZoom: number = this.maxzoom): void {
    const { buffer, tiles, tolerance, maxzoom, projection, indexMaxzoom } = this;
    const stack: bigint[] = [startID];
    // avoid recursion by using a processing queue
    while (stack.length > 0) {
      // find our next tile to split
      const stackID = stack.pop();
      if (stackID === undefined) break;
      const tile = tiles.get(stackID);
      // if the tile we need does not exist, is empty, or already transformed, skip it
      if (tile === undefined || tile.isEmpty() || tile.transformed) continue;
      const tileZoom = level(projection, tile.id);
      // 1: stop tiling if we reached a defined end
      // 2: stop tiling if it's the first-pass tiling, and we reached max zoom for indexing
      // 3: stop at currently needed maxzoom OR current tile does not include child
      if (
        tileZoom >= maxzoom || // 1
        (endID === undefined && tileZoom >= indexMaxzoom) || // 2
        (endID !== undefined && (tileZoom > endZoom || !contains(projection, tile.id, endID))) // 3
      )
        continue;

      // split the tile and store the children
      const [blID, brID, tlID, trID] = splitTile(tile, buffer);
      tiles.set(blID.id, blID);
      tiles.set(brID.id, brID);
      tiles.set(tlID.id, tlID);
      tiles.set(trID.id, trID);
      // now that the tile has been split, we can transform it
      tile.transform(tolerance, maxzoom);
      // push the new features to the stack
      stack.push(blID.id, brID.id, tlID.id, trID.id);
    }
  }

  /**
   * @param id - the tile id to acquire
   * @returns - the tile if it exists
   */
  getTile(id: bigint): undefined | Tile {
    const { tiles, projection, faces } = this;
    const zoom = level(projection, id);
    const face = getFace(projection, id);
    // If the zoom is out of bounds, return nothing
    if (zoom < 0 || zoom > 20 || !faces.has(face)) return;

    // we want to find the closest tile to the data.
    let pID = id;
    while (!tiles.has(pID) && !isFace(projection, pID)) {
      pID = parentID(projection, pID);
    }
    // split as necessary, the algorithm will know if the tile is already split
    this.splitTile(pID, id, zoom);

    return tiles.get(id);
  }
}
