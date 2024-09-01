import { level } from './id';
import { simplify } from '.';

import type { Projection, VectorFeature } from '.';

/** Tile Class to contain the tile information for splitting or simplifying */
export class Tile {
  /**
   * @param id - the tile id
   * @param projection - WM or S2
   * @param layers - the tile's layers
   * @param simplified - whether the tile feature geometry has been simplified
   */
  constructor(
    public id: bigint,
    public projection: Projection,
    public layers: Record<string, Layer> = {},
    public simplified = false,
  ) {}

  /**
   * @param feature - Vector Feature
   * @param layer - layer to store the feature to
   */
  addFeature(feature: VectorFeature, layer?: string): void {
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
  simplify(tolerance = 1, maxzoom = 16) {
    const zoom = level(this.projection, this.id);

    for (const layer of Object.values(this.layers)) {
      for (const feature of layer.features) {
        simplify(feature.geometry, tolerance, zoom, maxzoom);
      }
    }

    this.simplified = true;
  }
}

/** Layer Class to contain the layer information for splitting or simplifying */
export class Layer {
  /**
   * @param name - the layer name
   * @param features - the layer's features
   */
  constructor(
    public name: string,
    public features: VectorFeature[] = [],
  ) {}
}
