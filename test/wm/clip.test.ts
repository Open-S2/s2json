import { Tile } from '../../src';
import { fromFace } from '../../src/id';
import { clipLine, splitTile } from '../../src/wm/clip';
import { describe, expect, it, test } from 'bun:test';

import type { BBox, VectorFeature, VectorLineString } from '../../src';

test('clipLine - simple', () => {
  const bbox: BBox = [0, 0, 10.5, 10.5];
  const line: VectorLineString = [
    { x: 0, y: 0 },
    { x: 5, y: 5, z: 4, m: { a: 1 } },
    { x: 10, y: 10, z: -2, m: { a: 2 } },
    { x: 15, y: 15, z: 3, m: { a: 3 } },
  ];

  const res = clipLine(line, bbox, false, 0, 0);
  expect(res).toEqual([
    {
      line: [
        { m: undefined, x: 0, y: 0, z: undefined },
        { m: { a: 1 }, x: 5, y: 5, z: 4 },
        { m: { a: 2 }, x: 10, y: 10, z: -2 },
        { x: 10.5, y: 10.5, z: 0.5, m: { a: 3 } },
      ],
      offset: 0,
      vecBBox: [0, 0, 10.5, +10.5, +-2, +4],
    },
  ]);

  // polygon case:
  const res2 = clipLine(line, bbox, true, 0, 0);
  expect(res2).toEqual([
    {
      line: [
        { m: undefined, x: 0, y: 0, z: undefined },
        { m: { a: 1 }, x: 5, y: 5, z: 4 },
        { m: { a: 2 }, x: 10, y: 10, z: -2 },
        { x: 10.5, y: 10.5, z: 0.5, m: { a: 3 } },
        { m: undefined, x: 0, y: 0, z: undefined },
      ],
      offset: 0,
      vecBBox: [0, 0, 10.5, 10.5, -2, 4],
    },
  ]);
});

test('clipLine - starts outside left', () => {
  const bbox: BBox = [2.5, 2.5, 10.5, 10.5];
  const line: VectorLineString = [
    { x: 0, y: 0 },
    { x: 5, y: 5, z: 4, m: { a: 1 } },
    { x: 10, y: 10, z: -2, m: { a: 2 } },
    { x: 15, y: 15, z: 3, m: { a: 3 } },
  ];

  const res = clipLine(line, bbox, false, 0, 0.5);
  expect(res).toEqual([
    {
      line: [
        { m: { a: 1 }, x: 2, y: 2, z: 4 },
        { m: { a: 1 }, x: 5, y: 5, z: 4 },
        { m: { a: 2 }, x: 10, y: 10, z: -2 },
        { x: 11, y: 11, z: 0.5, m: { a: 3 } },
      ],
      offset: 2.8284271247461903,
      vecBBox: [2, 2, 11, 11, -2, 4],
    },
  ]);

  // polygon case:
  const res2 = clipLine(line, bbox, true, 0, 0.5);
  expect(res2).toEqual([
    {
      line: [
        { m: { a: 1 }, x: 2, y: 2, z: 4 },
        { m: { a: 1 }, x: 5, y: 5, z: 4 },
        { m: { a: 2 }, x: 10, y: 10, z: -2 },
        { x: 11, y: 11, z: 0.5, m: { a: 3 } },
        { m: { a: 1 }, x: 2, y: 2, z: 4 },
      ],
      offset: 2.8284271247461903,
      vecBBox: [2, 2, 11, 11, -2, 4],
    },
  ]);
});

test('clipLine - starts outside right', () => {
  const bbox: BBox = [2.5, 2.5, 10.5, 10.5];
  const line: VectorLineString = [
    { x: 15, y: 15, z: 3, m: { a: 3 } },
    { x: 10, y: 10, z: -2, m: { a: 2 } },
    { x: 5, y: 5, z: 4, m: { a: 1 } },
    { x: 0, y: 0 },
  ];

  const res = clipLine(line, bbox, false, 0, 0);
  expect(res).toEqual([
    {
      line: [
        { x: 10.5, y: 10.5, z: 0.5, m: { a: 2 } },
        { m: { a: 2 }, x: 10, y: 10, z: -2 },
        { m: { a: 1 }, x: 5, y: 5, z: 4 },
        { m: { a: 1 }, x: 2.5, y: 2.5, z: 4 },
      ],
      offset: 6.363961030678928,
      vecBBox: [2.5, 2.5, 10.5, 10.5, -2, 4],
    },
  ]);

  // polygon case:
  const res2 = clipLine(line, bbox, true, 0, 0);
  expect(res2).toEqual([
    {
      line: [
        { x: 10.5, y: 10.5, z: 0.5, m: { a: 2 } },
        { m: { a: 2 }, x: 10, y: 10, z: -2 },
        { m: { a: 1 }, x: 5, y: 5, z: 4 },
        { m: { a: 1 }, x: 2.5, y: 2.5, z: 4 },
        { x: 10.5, y: 10.5, z: 0.5, m: { a: 2 } },
      ],
      offset: 6.363961030678928,
      vecBBox: [2.5, 2.5, 10.5, 10.5, -2, 4],
    },
  ]);
});

test('clipLine - starts outside right and moves to outside left', () => {
  const bbox: BBox = [2.5, 2.5, 10.5, 10.5];
  const line: VectorLineString = [
    { x: 15, y: 15, z: 3, m: { a: 3 } },
    { x: 0, y: 0 },
  ];

  const res = clipLine(line, bbox, false, 0, 0);
  expect(res).toEqual([
    {
      line: [
        { x: 10.5, y: 10.5, z: 3, m: undefined },
        { m: { a: 3 }, x: 2.5, y: 2.5, z: 3 },
      ],
      offset: 6.363961030678928,
      vecBBox: [2.5, 2.5, 10.5, 10.5, 3, 3],
    },
  ]);
});

test('clipLine - only vertically', () => {
  const bbox: BBox = [2.5, 2.5, 10.5, 10.5];
  const line: VectorLineString = [
    { x: 4, y: 0 },
    { x: 5, y: 5, z: 4, m: { a: 1 } },
    { x: 7, y: 10, z: -2, m: { a: 2 } },
    { x: 9, y: 15, z: 3, m: { a: 3 } },
  ];

  const res = clipLine(line, bbox, false, 0, 0);
  expect(res).toEqual([
    {
      line: [
        { m: { a: 1 }, x: 4.5, y: 2.5, z: 4 },
        { m: { a: 1 }, x: 5, y: 5, z: 4 },
        { m: { a: 2 }, x: 7, y: 10, z: -2 },
        { m: { a: 3 }, x: 7.2, y: 10.5, z: 0.5 },
      ],
      offset: 2.5495097567963922,
      vecBBox: [4.5, 2.5, 7.2, 10.5, -2, 4],
    },
  ]);

  const res2 = clipLine(line, bbox, true, 0, 0);
  expect(res2).toEqual([
    {
      line: [
        { m: { a: 1 }, x: 4.5, y: 2.5, z: 4 },
        { m: { a: 1 }, x: 5, y: 5, z: 4 },
        { m: { a: 2 }, x: 7, y: 10, z: -2 },
        { m: { a: 3 }, x: 7.2, y: 10.5, z: 0.5 },
        { m: undefined, x: 7.5, y: 10.5, z: 3 },
        { m: { a: 3 }, x: 4.833333333333333, y: 2.5, z: 3 },
        { m: { a: 1 }, x: 4.5, y: 2.5, z: 4 },
      ],
      offset: 2.5495097567963922,
      vecBBox: [4.5, 2.5, 7.5, 10.5, -2, 4],
    },
  ]);
});

test('clipLine - passes through the x axis from left to right, then again right to left', () => {
  const bbox: BBox = [0, 0, 10, 10];
  const line: VectorLineString = [
    { x: -2, y: 4 },
    { x: 2, y: 4 },
    { x: 8, y: 4 },
    { x: 12, y: 4 },
    { x: 12, y: 8 },
    { x: 8, y: 8 },
    { x: 2, y: 8 },
    { x: -2, y: 8 },
  ];

  const res = clipLine(line, bbox, false, 0, 0);
  expect(res).toEqual([
    {
      line: [
        { x: 0, y: 4 },
        { x: 2, y: 4 },
        { x: 8, y: 4 },
        { x: 10, y: 4 },
      ],
      offset: 2,
      vecBBox: [0, 4, 10, 4],
    },
    {
      line: [
        { x: 10, y: 8 },
        { x: 8, y: 8 },
        { x: 2, y: 8 },
        { x: 0, y: 8 },
      ],
      offset: 20,
      vecBBox: [0, 8, 10, 8],
    },
  ]);
});

describe('splitTile', () => {
  it('Point', () => {
    const faceID = fromFace('WM', 0);
    const features: VectorFeature[] = [
      {
        type: 'VectorFeature',
        properties: { a: 2 },
        geometry: {
          type: 'Point',
          coordinates: { x: 0.25, y: 0.25 },
          vecBBox: [0.25, 0.25, 0.25, 0.25],
        },
      },
      {
        type: 'VectorFeature',
        properties: { a: 2 },
        geometry: {
          type: 'Point',
          coordinates: { x: 0.75, y: 0.75 },
          vecBBox: [0.75, 0.75, 0.75, 0.75],
        },
      },
      {
        type: 'VectorFeature',
        properties: { a: 2 },
        geometry: {
          type: 'Point',
          coordinates: { x: 0.75, y: 0.25 },
          vecBBox: [0.75, 0.25, 0.75, 0.25],
        },
      },
      {
        type: 'VectorFeature',
        properties: { a: 2 },
        geometry: {
          type: 'Point',
          coordinates: { x: 0.25, y: 0.75 },
          vecBBox: [0.25, 0.75, 0.25, 0.75],
        },
      },
    ];

    const tile = new Tile(faceID, 'WM');
    for (const feature of features) tile.addFeature(feature);

    const res = splitTile(tile);
    expect(res).toEqual([
      {
        id: 288230376151711744n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  coordinates: {
                    x: 0.25,
                    y: 0.25,
                  },
                  type: 'Point',
                  vecBBox: [0.25, 0.25, 0.25, 0.25],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376688582656n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  coordinates: {
                    x: 0.75,
                    y: 0.25,
                  },
                  type: 'Point',
                  vecBBox: [0.75, 0.25, 0.75, 0.25],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376151711745n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  coordinates: {
                    x: 0.25,
                    y: 0.75,
                  },
                  type: 'Point',
                  vecBBox: [0.25, 0.75, 0.25, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376688582657n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  coordinates: {
                    x: 0.75,
                    y: 0.75,
                  },
                  type: 'Point',
                  vecBBox: [0.75, 0.75, 0.75, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
    ]);
  });
  it('MultiPoint', () => {
    const faceID = fromFace('WM', 0);
    const features: VectorFeature[] = [
      {
        type: 'VectorFeature',
        properties: { a: 2 },
        geometry: {
          type: 'MultiPoint',
          coordinates: [
            { x: 0.25, y: 0.25 },
            { x: 0.75, y: 0.75 },
            { x: 0.75, y: 0.25 },
            { x: 0.25, y: 0.75 },
          ],
          vecBBox: [0.25, 0.25, 0.75, 0.75],
        },
      },
    ];

    const tile = new Tile(faceID, 'WM');
    for (const feature of features) tile.addFeature(feature);

    const res = splitTile(tile);
    expect(res).toEqual([
      {
        id: 288230376151711744n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [{ x: 0.25, y: 0.25 }],
                  type: 'MultiPoint',
                  vecBBox: [0.25, 0.25, 0.25, 0.25],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376688582656n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [{ x: 0.75, y: 0.25 }],
                  type: 'MultiPoint',
                  vecBBox: [0.75, 0.25, 0.75, 0.25],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376151711745n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [{ x: 0.25, y: 0.75 }],
                  type: 'MultiPoint',
                  vecBBox: [0.25, 0.75, 0.25, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376688582657n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [{ x: 0.75, y: 0.75 }],
                  type: 'MultiPoint',
                  vecBBox: [0.75, 0.75, 0.75, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
    ]);

    const splitAgain = splitTile(res[3]);

    expect(splitAgain).toEqual([
      {
        id: 576460753377165314n,
        layers: {},
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 576460753914036226n,
        layers: {},
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 576460753377165315n,
        layers: {},
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 576460753914036227n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [{ x: 0.75, y: 0.75 }],
                  type: 'MultiPoint',
                  vecBBox: [0.75, 0.75, 0.75, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
    ]);
  });

  it('LineString', () => {
    const faceID = fromFace('WM', 0);
    const features: VectorFeature[] = [
      {
        type: 'VectorFeature',
        properties: { a: 2 },
        geometry: {
          type: 'LineString',
          coordinates: [
            { x: 0.25, y: 0.25 },
            { x: 0.75, y: 0.75 },
            { x: 0.75, y: 0.25 },
            { x: 0.25, y: 0.75 },
          ],
          vecBBox: [0.25, 0.25, 0.75, 0.75],
        },
      },
    ];

    const tile = new Tile(faceID, 'WM');
    for (const feature of features) tile.addFeature(feature);

    const res = splitTile(tile);
    expect(res).toEqual([
      {
        id: 288230376151711744n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      { x: 0.25, y: 0.25 },
                      { x: 0.5625, y: 0.5625 },
                    ],
                    [
                      { x: 0.5625, y: 0.4375 },
                      { x: 0.4375, y: 0.5625 },
                    ],
                  ],
                  offset: [0, 1.4722718241315027],
                  type: 'MultiLineString',
                  vecBBox: [0.25, 0.25, 0.5625, 0.5625],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376688582656n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      { x: 0.4375, y: 0.4375 },
                      { x: 0.5625, y: 0.5625 },
                    ],
                    [
                      { x: 0.75, y: 0.5625 },
                      { x: 0.75, y: 0.25 },
                      { x: 0.4375, y: 0.5625 },
                    ],
                  ],
                  offset: [0.2651650429449553, 0.8946067811865475],
                  type: 'MultiLineString',
                  vecBBox: [0.4375, 0.25, 0.75, 0.5625],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376151711745n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      { x: 0.4375, y: 0.4375 },
                      { x: 0.5625, y: 0.5625 },
                    ],
                    [
                      { x: 0.5625, y: 0.4375 },
                      { x: 0.25, y: 0.75 },
                    ],
                  ],
                  offset: [0.2651650429449553, 1.4722718241315027],
                  type: 'MultiLineString',
                  vecBBox: [0.25, 0.4375, 0.5625, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376688582657n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      { x: 0.4375, y: 0.4375 },
                      { x: 0.75, y: 0.75 },
                      { x: 0.75, y: 0.4375 },
                    ],
                    [
                      { x: 0.5625, y: 0.4375 },
                      { x: 0.4375, y: 0.5625 },
                    ],
                  ],
                  offset: [0.2651650429449553, 1.4722718241315027],
                  type: 'MultiLineString',
                  vecBBox: [0.4375, 0.4375, 0.75, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
    ]);
  });

  it('MultiLineString', () => {
    const faceID = fromFace('WM', 0);
    const features: VectorFeature[] = [
      {
        type: 'VectorFeature',
        properties: { a: 2 },
        geometry: {
          type: 'MultiLineString',
          coordinates: [
            [
              { x: 0.25, y: 0.25 },
              { x: 0.75, y: 0.25 },
              { x: 0.75, y: 0.75 },
              { x: 0.25, y: 0.75 },
            ],
            [
              { x: 0.4, y: 0.4 },
              { x: 0.6, y: 0.4 },
              { x: 0.6, y: 0.6 },
              { x: 0.4, y: 0.6 },
            ],
          ],
          vecBBox: [0.25, 0.25, 0.75, 0.75],
        },
      },
    ];

    const tile = new Tile(faceID, 'WM');
    for (const feature of features) tile.addFeature(feature);

    const res = splitTile(tile);
    expect(res).toEqual([
      {
        id: 288230376151711744n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      { x: 0.25, y: 0.25 },
                      { x: 0.5625, y: 0.25 },
                    ],
                    [
                      { x: 0.4, y: 0.4 },
                      { x: 0.5625, y: 0.4 },
                    ],
                  ],
                  offset: [0, 0],
                  type: 'MultiLineString',
                  vecBBox: [0.25, 0.25, 0.5625, 0.5625],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376688582656n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      { x: 0.4375, y: 0.25 },
                      { x: 0.75, y: 0.25 },
                      { x: 0.75, y: 0.5625 },
                    ],
                    [
                      { x: 0.4375, y: 0.4 },
                      { x: 0.6, y: 0.4 },
                      { x: 0.6, y: 0.5625 },
                    ],
                  ],
                  offset: [0.1875, 0.03749999999999998],
                  type: 'MultiLineString',
                  vecBBox: [0.4375, 0.25, 0.75, 0.5625],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376151711745n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      { x: 0.5625, y: 0.75 },
                      { x: 0.25, y: 0.75 },
                    ],
                    [
                      { x: 0.5625, y: 0.6 },
                      { x: 0.4, y: 0.6 },
                    ],
                  ],
                  offset: [1.1875, 0.4374999999999999],
                  type: 'MultiLineString',
                  vecBBox: [0.25, 0.4375, 0.5625, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376688582657n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      { x: 0.75, y: 0.4375 },
                      { x: 0.75, y: 0.75 },
                      { x: 0.4375, y: 0.75 },
                    ],
                    [
                      { x: 0.6, y: 0.4375 },
                      { x: 0.6, y: 0.6 },
                      { x: 0.4375, y: 0.6 },
                    ],
                  ],
                  offset: [0.6875, 0.23749999999999993],
                  type: 'MultiLineString',
                  vecBBox: [0.4375, 0.4375, 0.75, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
    ]);
  });

  it('Polygon', () => {
    const faceID = fromFace('WM', 0);
    const features: VectorFeature[] = [
      {
        type: 'VectorFeature',
        properties: { a: 2 },
        geometry: {
          type: 'Polygon',
          coordinates: [
            [
              { x: 0.25, y: 0.25 },
              { x: 0.75, y: 0.25 },
              { x: 0.75, y: 0.75 },
              { x: 0.25, y: 0.75 },
            ],
            [
              { x: 0.4, y: 0.6 },
              { x: 0.6, y: 0.6 },
              { x: 0.6, y: 0.4 },
              { x: 0.4, y: 0.4 },
            ],
          ],
          vecBBox: [0.25, 0.25, 0.75, 0.75],
        },
      },
    ];

    const tile = new Tile(faceID, 'WM');
    for (const feature of features) tile.addFeature(feature);

    const res = splitTile(tile);
    expect(res).toEqual([
      {
        id: 288230376151711744n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      { x: 0.25, y: 0.25 },
                      { x: 0.5625, y: 0.25 },
                      { x: 0.5625, y: 0.5625 },
                      { x: 0.25, y: 0.5625 },
                      { x: 0.25, y: 0.25 },
                    ],
                    [
                      { x: 0.5625, y: 0.5625 },
                      { x: 0.5625, y: 0.4 },
                      { x: 0.4, y: 0.4 },
                      { x: 0.4, y: 0.5625 },
                      { x: 0.5625, y: 0.5625 },
                    ],
                  ],
                  offset: [2.5, 0.6374999999999998],
                  type: 'Polygon',
                  vecBBox: [0.25, 0.25, 0.5625, 0.5625],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376688582656n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      { x: 0.4375, y: 0.25 },
                      { x: 0.75, y: 0.25 },
                      { x: 0.75, y: 0.5625 },
                      { x: 0.4375, y: 0.5625 },
                      { x: 0.4375, y: 0.25 },
                    ],
                    [
                      { x: 0.6, y: 0.5625 },
                      { x: 0.6, y: 0.4 },
                      { x: 0.4375, y: 0.4 },
                      { x: 0.4375, y: 0.5625 },
                      { x: 0.6, y: 0.5625 },
                    ],
                  ],
                  offset: [1.5, 0.23749999999999993],
                  type: 'Polygon',
                  vecBBox: [0.4375, 0.25, 0.75, 0.5625],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376151711745n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      { x: 0.5625, y: 0.4375 },
                      { x: 0.5625, y: 0.75 },
                      { x: 0.25, y: 0.75 },
                      { x: 0.25, y: 0.4375 },
                      { x: 0.5625, y: 0.4375 },
                    ],
                    [
                      { x: 0.4, y: 0.6 },
                      { x: 0.5625, y: 0.6 },
                      { x: 0.5625, y: 0.4375 },
                      { x: 0.4, y: 0.4375 },
                      { x: 0.4, y: 0.6 },
                    ],
                  ],
                  offset: [1.6875, 0.9999999999999998],
                  type: 'Polygon',
                  vecBBox: [0.25, 0.4375, 0.5625, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376688582657n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      { x: 0.75, y: 0.4375 },
                      { x: 0.75, y: 0.75 },
                      { x: 0.4375, y: 0.75 },
                      { x: 0.4375, y: 0.4375 },
                      { x: 0.75, y: 0.4375 },
                    ],
                    [
                      { x: 0.4375, y: 0.6 },
                      { x: 0.6, y: 0.6 },
                      { x: 0.6, y: 0.4375 },
                      { x: 0.4375, y: 0.4375 },
                      { x: 0.4375, y: 0.6 },
                    ],
                  ],
                  offset: [0.6875, 0.5999999999999999],
                  type: 'Polygon',
                  vecBBox: [0.4375, 0.4375, 0.75, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
    ]);
  });

  it('MultiPolygon', () => {
    const faceID = fromFace('WM', 0);
    const features: VectorFeature[] = [
      {
        type: 'VectorFeature',
        properties: { a: 2 },
        geometry: {
          type: 'MultiPolygon',
          coordinates: [
            [
              [
                { x: 0.25, y: 0.25 },
                { x: 0.75, y: 0.25 },
                { x: 0.75, y: 0.75 },
                { x: 0.25, y: 0.75 },
              ],
              [
                { x: 0.4, y: 0.6 },
                { x: 0.6, y: 0.6 },
                { x: 0.6, y: 0.4 },
                { x: 0.4, y: 0.4 },
              ],
            ],
          ],
          vecBBox: [0.25, 0.25, 0.75, 0.75],
        },
      },
    ];

    const tile = new Tile(faceID, 'WM');
    for (const feature of features) tile.addFeature(feature);

    const res = splitTile(tile);
    expect(res).toEqual([
      {
        id: 288230376151711744n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      [
                        { x: 0.25, y: 0.25 },
                        { x: 0.5625, y: 0.25 },
                        { x: 0.5625, y: 0.5625 },
                        { x: 0.25, y: 0.5625 },
                        { x: 0.25, y: 0.25 },
                      ],
                      [
                        { x: 0.5625, y: 0.5625 },
                        { x: 0.5625, y: 0.4 },
                        { x: 0.4, y: 0.4 },
                        { x: 0.4, y: 0.5625 },
                        { x: 0.5625, y: 0.5625 },
                      ],
                    ],
                  ],
                  offset: [[1.3125, 0.19999999999999996]],
                  type: 'MultiPolygon',
                  vecBBox: [0.25, 0.25, 0.5625, 0.5625],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376688582656n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      [
                        { x: 0.4375, y: 0.25 },
                        { x: 0.75, y: 0.25 },
                        { x: 0.75, y: 0.5625 },
                        { x: 0.4375, y: 0.5625 },
                        { x: 0.4375, y: 0.25 },
                      ],
                      [
                        { x: 0.6, y: 0.5625 },
                        { x: 0.6, y: 0.4 },
                        { x: 0.4375, y: 0.4 },
                        { x: 0.4375, y: 0.5625 },
                        { x: 0.6, y: 0.5625 },
                      ],
                    ],
                  ],
                  offset: [[1.3125, 0.19999999999999996]],
                  type: 'MultiPolygon',
                  vecBBox: [0.4375, 0.25, 0.75, 0.5625],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376151711745n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      [
                        { x: 0.5625, y: 0.4375 },
                        { x: 0.5625, y: 0.75 },
                        { x: 0.25, y: 0.75 },
                        { x: 0.25, y: 0.4375 },
                        { x: 0.5625, y: 0.4375 },
                      ],
                      [
                        { x: 0.4, y: 0.6 },
                        { x: 0.5625, y: 0.6 },
                        { x: 0.5625, y: 0.4375 },
                        { x: 0.4, y: 0.4375 },
                        { x: 0.4, y: 0.6 },
                      ],
                    ],
                  ],
                  offset: [[0.5, 0.5624999999999999]],
                  type: 'MultiPolygon',
                  vecBBox: [0.25, 0.4375, 0.5625, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
      {
        id: 288230376688582657n,
        layers: {
          default: {
            features: [
              {
                geometry: {
                  bbox: undefined,
                  coordinates: [
                    [
                      [
                        { x: 0.75, y: 0.4375 },
                        { x: 0.75, y: 0.75 },
                        { x: 0.4375, y: 0.75 },
                        { x: 0.4375, y: 0.4375 },
                        { x: 0.75, y: 0.4375 },
                      ],
                      [
                        { x: 0.4375, y: 0.6 },
                        { x: 0.6, y: 0.6 },
                        { x: 0.6, y: 0.4375 },
                        { x: 0.4375, y: 0.4375 },
                        { x: 0.4375, y: 0.6 },
                      ],
                    ],
                  ],
                  offset: [[0.5, 0.5624999999999999]],
                  type: 'MultiPolygon',
                  vecBBox: [0.4375, 0.4375, 0.75, 0.75],
                },
                properties: {
                  a: 2,
                },
                type: 'VectorFeature',
              },
            ],
            name: 'default',
          },
        },
        projection: 'WM',
        simplified: false,
      } as unknown as Tile,
    ]);
  });
});