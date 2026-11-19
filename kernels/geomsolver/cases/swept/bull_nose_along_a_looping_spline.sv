// A block, the box from (-10, -10, -8) to (30, 25, 0), its top in the top plane; an upright
// bull-nose mill, radius 2 from its flat bottom 1.5 below the top up to 5 above it, the corner of
// its bottom rounded to 0.5, a turn of its half-section in the front plane, its axis on the
// vertical line through the start of the cubic Bezier through (0, 0), (30, 20), (-10, 20) and
// (20, 0) of the top face. A frame riding the curve carries it, upright, along the whole curve,
// and everything it passes through is cut from the block.
unit mm
use std
use std.solids
use std.solids (cut)
use std.stations

in std.top {
  a := point
  b := point hint((30, -10))
  c := point hint((30, 25))
  d := point hint((-10, 25))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-10, -10)) a
  a distance(40) b
  b distance(35) c

  // the path, a spline over its four control points
  k0 := point
  k1 := point hint((30, 20))
  k2 := point hint((-10, 20))
  k3 := point hint((20, 0))
  fix((0, 0)) k0
  fix((30, 20)) k1
  fix((-10, 20)) k2
  fix((20, 0)) k3
  route := spline(k0, k1, k2, k3)
}
part := solid(std.solids.Prism(outline, from: -8mm, to: 0mm))

in std.front {
  e0 := point
  q := point hint((2, -1.5))
  e2 := point hint((2, 5))
  e3 := point hint((0, 5))
  fix((0, -1.5)) e0
  e0 distance(2) q
  fix((0, 5)) e3
  // the side meets the flat bottom at q, the corner rounded to 0.5
  half := horizontal line(e0, q) -> round(r: 0.5mm) vertical line(q, e2) ->
          horizontal line(e2, e3) -> vertical (spindle := line(e3, e0)) -> close
}
mill := solid(std.solids.Revolution(half, about: spindle))

// the frame that carries it along the whole spline
along := std.stations.Along(route, up: std.z)
groove := solid({ p | some t in (0rad, 1rad): p in mill at(under: along.f) t })
groove cut part
