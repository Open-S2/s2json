# s2json [![npm][npm-image]][npm-url] [![crate][crate-image]][crate-url] [![downloads][downloads-image]][downloads-url] [![bundle][bundle-image]][bundle-url] [![docs-ts][docs-ts-image]][docs-ts-url] [![docs-rust][docs-rust-image]][docs-rust-url] ![doc-coverage][doc-coverage-image] [![Discord][discord-image]][discord-url]

[npm-image]: https://img.shields.io/npm/v/s2json-spec.svg?logo=npm&logoColor=white
[npm-url]: https://npmjs.org/package/s2json-spec
[crate-image]: https://img.shields.io/crates/v/s2json.svg?logo=rust&logoColor=white
[crate-url]: https://crates.io/crates/s2json
[bundle-image]: https://img.shields.io/bundlejs/size/s2json-spec?exports=VectorTile
[bundle-url]: https://bundlejs.com/?q=s2json-spec
[downloads-image]: https://img.shields.io/npm/dm/s2json-spec.svg
[downloads-url]: https://www.npmjs.com/package/s2json-spec
[docs-ts-image]: https://img.shields.io/badge/docs-typescript-yellow.svg
[docs-ts-url]: https://open-s2.github.io/s2json/
[docs-rust-image]: https://img.shields.io/badge/docs-rust-yellow.svg
[docs-rust-url]: https://docs.rs/s2json
[doc-coverage-image]: https://raw.githubusercontent.com/Open-S2/s2json/master/assets/doc-coverage.svg
[discord-image]: https://img.shields.io/discord/953563031701426206?logo=discord&logoColor=white
[discord-url]: https://discord.opens2.com

## About

S2JSON is a format for encoding a variety of geographic data structures that simplifies the GeoJSON spec and builds ontop of it to include S2 Geometry.

Notable features of S2JSON are:

* Properties data is clearly defined on how it can be shaped.
* 🧊 Support for 3D geometries.
* ♏ Support for M-Values for each geometry point.
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
// S2JSON example
{
  "type": "S2Feature",
  "face": 0,
  "geometry": {
    "type": "Point",
    "coordinates": [0.5, 0.5]
  },
  "properties": {
    "name": "Null Island"
  }
}
```

## Read The Spec

[s2json-spec](/s2json-spec/1.0.0/README.md)

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
