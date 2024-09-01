const tolerance = Math.pow(3 / ((1 << 16) * 8_192), 2)
const coords = [0.25, 0.25, 1, 0.75, 0.75, 0, 0.75, 0.25, 0, 0.25, 0.75, 1]
simplify(coords, 0, coords.length - 3, tolerance)
console.log(coords)

const t2 = 3 / ((1 << 0) * 8_192)

const simplified = []
addLine(simplified, coords, t2, false, false)
console.log('simplified', simplified)

/**
 * calculate simplification of line vector data using
 * optimized Douglas-Peucker algorithm
 */
export default function simplify (
  coords: number[],
  first: number,
  last: number,
  sqTolerance: number
): void {
  let maxSqDist = sqTolerance
  const mid = (last - first) >> 1
  let minPosToMid = last - first
  let index: undefined | number

  const as = coords[first]
  const at = coords[first + 1]
  const bs = coords[last]
  const bt = coords[last + 1]

  for (let i = first + 3; i < last; i += 3) {
    const d = getSqSegDist(coords[i], coords[i + 1], as, at, bs, bt)

    if (d > maxSqDist) {
      index = i
      maxSqDist = d
    } else if (d === maxSqDist) {
      // a workaround to ensure we choose a pivot close to the middle of the list,
      // reducing recursion depth, for certain degenerate inputs
      const posToMid = Math.abs(i - mid)
      if (posToMid < minPosToMid) {
        index = i
        minPosToMid = posToMid
      }
    }
  }

  if (index !== undefined && maxSqDist > sqTolerance) {
    if (index - first > 3) simplify(coords, first, index, sqTolerance)
    coords[index + 2] = maxSqDist
    if (last - index > 3) simplify(coords, index, last, sqTolerance)
  }
}

// square distance from a point to a segment
function getSqSegDist (
  ps: number,
  pt: number,
  s: number,
  t: number,
  bs: number,
  bt: number
): number {
  let ds = bs - s
  let dt = bt - t

  if (ds !== 0 || dt !== 0) {
    const m = ((ps - s) * ds + (pt - t) * dt) / (ds * ds + dt * dt)

    if (m > 1) {
      s = bs
      t = bt
    } else if (m > 0) {
      s += ds * m
      t += dt * m
    }
  }

  ds = ps - s
  dt = pt - t

  return ds * ds + dt * dt
}

function addLine (
  result: number[][],
  geom: number[],
  tolerance: number,
  isPolygon: boolean,
  isOuter: boolean
): void {
  const sqTolerance = tolerance * tolerance
  const size = geom.length / 3
  if (tolerance > 0 && (size < (isPolygon ? sqTolerance : tolerance))) {
    return
  }

  const ring: number[] = []

  for (let i = 0; i < geom.length; i += 3) {
    if (tolerance === 0 || geom[i + 2] > sqTolerance) {
      ring.push(geom[i])
      ring.push(geom[i + 1])
    }
  }

  if (isPolygon) rewind(ring, isOuter)
  result.push(ring)
}

function rewind (ring: number[], clockwise: boolean): void {
  let area = 0
  for (let i = 0, len = ring.length, j = len - 2; i < len; j = i, i += 2) {
    area += (ring[i] - ring[j]) * (ring[i + 1] + ring[j + 1])
  }
  if ((area > 0) === clockwise) {
    for (let i = 0, len = ring.length; i < len / 2; i += 2) {
      const s = ring[i]
      const t = ring[i + 1]
      ring[i] = ring[len - 2 - i]
      ring[i + 1] = ring[len - 1 - i]
      ring[len - 2 - i] = s
      ring[len - 1 - i] = t
    }
  }
}
