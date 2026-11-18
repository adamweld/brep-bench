// A block, the box from (-6, -6, -8) to (36, 26, 0), its top in the top plane; a square key, the
// box from (-1.5, -0.8, -1.5) to (1.5, 0.8, 2.5), drawn where the cubic Bezier through (0, 0),
// (10, 0), (20, 20) and (30, 20) starts and carried rigidly by a frame riding the curve (its x the
// tangent, its z upright), and everything the key passes through cut from the block.
unit mm
use std
use std.solids
use std.solids (cut)
use std.stations

in std.top {
  a := point
  b := point hint((36, -6))
  c := point hint((36, 26))
  d := point hint((-6, 26))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-6, -6)) a
  a distance(42) b
  b distance(32) c

  // the path, a spline over its four control points
  k0 := point
  k1 := point hint((10, 0))
  k2 := point hint((20, 20))
  k3 := point hint((30, 20))
  fix((0, 0)) k0
  fix((10, 0)) k1
  fix((20, 20)) k2
  fix((30, 20)) k3
  route := spline(k0, k1, k2, k3)

  // the key's footprint, 3 along the curve's start and 1.6 across it
  e := point
  f := point hint((1.5, -0.8))
  g := point hint((1.5, 0.8))
  h := point hint((-1.5, 0.8))
  footprint := horizontal line(e, f) -> vertical line(f, g) ->
               horizontal line(g, h) -> vertical line(h, e) -> close
  fix((-1.5, -0.8)) e
  e distance(3) f
  f distance(1.6) g
}
part := solid(std.solids.Prism(outline, from: -8mm, to: 0mm))

key := solid(std.solids.Prism(footprint, from: -1.5mm, to: 2.5mm))
along := std.stations.Along(route, up: std.z)
groove := solid({ p | some t in (0rad, 1rad): p in key at(under: along.f) t })
groove cut part
