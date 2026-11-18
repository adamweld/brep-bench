// A block, the box from (-6, -6, -8) to (36, 26, 0), its top in the top plane; a bull-nose mill
// leaning 30 degrees toward +x, drawn where the cubic Bezier through (0, 0), (10, 0), (20, 20) and
// (30, 20) starts, carried rigidly by a frame riding the curve (its x the tangent, its z upright),
// and everything the mill passes through cut from the block. The mill is a turn of its
// half-section, drawn in the plane through its axis: radius 2 from the middle of its flat end at
// (0, 0, -1) up to 3 along the axis, the corner of its end rounded to 0.5.
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

  k0 := point
  k1 := point hint((10, 0))
  k2 := point hint((20, 20))
  k3 := point hint((30, 20))
  fix((0, 0)) k0
  fix((10, 0)) k1
  fix((20, 20)) k2
  fix((30, 20)) k3
  route := spline(k0, k1, k2, k3)
}
part := solid(std.solids.Prism(outline, from: -8mm, to: 0mm))

// the mill's axis leans 30 degrees from upright, turned about y from z toward x; its section is
// drawn in the plane through that axis and the x direction, square to y
axle := axis hint(dir: (0.5, 0, 0.866))
axle perpendicular std.y
std.z angle(30deg, about: std.y) axle
across := axis hint(dir: (0.866, 0, -0.5))
across perpendicular std.y
std.x angle(30deg, about: std.y) across
bench := plane(u: across, v: axle)
fix(origin == (0, 0, -1)) bench
in bench {
  e0 := point
  q := point hint((2, 0))
  e2 := point hint((2, 3))
  e3 := point hint((0, 3))
  fix((0, 0)) e0
  e0 distance(2) q
  e0 distance(3) e3
  // the side meets the flat end at q, the corner rounded to 0.5
  section := horizontal line(e0, q) -> round(r: 0.5mm) vertical line(q, e2) ->
             horizontal line(e2, e3) -> vertical (spindle := line(e3, e0)) -> close
}
mill := solid(std.solids.Revolution(section, about: spindle))

along := std.stations.Along(route, up: std.z)
groove := solid({ p | some t in (0rad, 1rad): p in mill at(under: along.f) t })
groove cut part
