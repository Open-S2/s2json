<h1 style="text-align: center;">
  <div align="center">s2json</div>
</h1>

<p align="center">
  <a href="https://img.shields.io/github/actions/workflow/status/Open-S2/s2json-spec/test.yml?logo=github">
    <img src="https://img.shields.io/github/actions/workflow/status/Open-S2/s2json-spec/test.yml?logo=github" alt="GitHub Actions Workflow Status">
  </a>
  <a href="https://npmjs.org/package/s2json-spec">
    <img src="https://img.shields.io/npm/v/s2json-spec.svg?logo=npm&logoColor=white" alt="npm">
  </a>
  <a href="https://crates.io/crates/s2json">
    <img src="https://img.shields.io/crates/v/s2json.svg?logo=rust&logoColor=white" alt="crate">
  </a>
  <a href="https://www.npmjs.com/package/s2json-spec">
    <img src="https://img.shields.io/npm/dm/s2json-spec.svg" alt="downloads">
  </a>
  <a href="https://bundlejs.com/?q=s2json-spec">
    <img src="https://img.shields.io/bundlejs/size/s2json-spec" alt="bundle">
  </a>
  <a href="https://open-s2.github.io/s2json/">
    <img src="https://img.shields.io/badge/docs-typescript-yellow.svg" alt="docs-ts">
  </a>
  <a href="https://docs.rs/s2json">
    <img src="https://img.shields.io/badge/docs-rust-yellow.svg" alt="docs-rust">
  </a>
  <a href="https://coveralls.io/github/Open-S2/s2json-spec?branch=master">
    <img src="https://coveralls.io/repos/github/Open-S2/s2json-spec/badge.svg?branch=master" alt="code-coverage">
  </a>
  <a href="https://discord.opens2.com">
    <img src="https://img.shields.io/discord/953563031701426206?logo=discord&logoColor=white" alt="Discord">
  </a>
</p>

## About

S2JSON is a format for encoding a variety of geographic data structures that simplifies the GeoJSON spec and builds ontop of it to include S2 Geometry.

Notable features of S2JSON are:

* Properties data is clearly defined on how it can be shaped.
* 🧊 Support for 3D geometries.
* 🏷️ Support for metadata.
* ♏ Support for M-Values for each geometry point.
* 📦 Support for bounding boxes
* 🫥 Updated spec to handle vector structures.
* 🔨 Tools for converting between GeoJSON, S2JSON, and quad-tree Tile structures
* 🪩 Support for `S2Feature` and `S2FeatureCollection` types based upon the S2 Geometry *spherical projection*.
* ♻️ Feature Properties & M-Values are defined in scope to ensure they can be easily processed by lower level languages as structures, but also adds value to other projects down the line.
* 🛑 GeoJSON no longer supports `GeometryCollection`.
* 📝 Attribution can be added to either a `FeatureCollection` or `S2FeatureCollection`

```json
// GeoJSON example
{
  "type": "Feature",
  "geometry": {
    "type": "Point",
    "coordinates": [125.6, 10.1]
  },
  "properties": {
    "name": "Dinagat Islands"
  }
}
// GeoJSON Vector example
{
  "type": "VectorFeature",
  "geometry": {
    "type": "Point",
    "coordinates": { x: 125.6, y: 10.1 }
  },
  "properties": {
    "name": "Dinagat Islands"
  }
}
// S2JSON example
{
  "type": "S2Feature",
  "face": 0,
  "geometry": {
    "type": "Point",
    "coordinates": { x: 0.5, y: 0.5 }
  },
  "properties": {
    "name": "Null Island"
  }
}
```

## Read The Spec

[s2json-spec](/s2json-spec/1.0.0/README.md)

## Implementations

* [s2-tools](https://github.com/Open-S2/s2-tools)

## Install

```bash
# bun
bun add -D s2json-spec
# pnpm
pnpm add -D s2json-spec
# yarn
yarn add -D s2json-spec
# npm
npm install -D s2json-spec

# cargo
cargo install s2json --dev
```

grammars/highlighting for VSCode are [available for install](https://marketplace.visualstudio.com/items?itemName=OpenS2.s2json-spec).

---

## Development

### Requirements

You need the tool `tarpaulin` to generate the coverage report. Install it using the following command:

```bash
cargo install cargo-tarpaulin
```

The `bacon coverage` tool is used to generate the coverage report. To utilize the [pycobertura](https://pypi.org/project/pycobertura/) package for a prettier coverage report, install it using the following command:

```bash
pip install pycobertura
```

### Validated Your Data

> Note: Be sure to set `resolveJsonModule: true` in your `tsconfig.json` to ensure json may be loaded as a module.

```ts
import Ajv from 'ajv';
import * as schema from 's2json-spec/s2json.schema.json'; // Path to the schema

import type { Feature } from 's2json-spec';

const ajv = new Ajv();
const validate = ajv.compile(schema);

const feature: Feature = {
  type: 'Feature',
  geometry: {
    type: 'Point',
    coordinates: [125.6, 10.1]
  },
  properties: {
    name: 'Dinagat Islands'
  },
};

validate(feature); // true
```

### Running Tests

To run the tests, use the following command:

```bash
# TYPESCRIPT
## basic test
bun run test
## live testing
bun run test:dev

# RUST
## basic test
cargo test
# live testing
bacon test
```

### Generating Coverage Report

To generate the coverage report, use the following command:

```bash
cargo tarpaulin
# bacon
bacon coverage # or type `l` inside the tool
```
