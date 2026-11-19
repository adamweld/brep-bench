// A block, the box from (-8, -8, -8) to (8, 8, 0), its top in the top plane; a bull-nose mill
// leaning 30 degrees from upright toward +x, the middle of its flat end at (0, 0, 4), pushed
// without turning 8 straight down so that it stops wholly inside the block, and everything it
// passes through cut from the block. The mill is a turn of its half-section, drawn in the plane
// through its axis: radius 2 from the middle of its flat end up to 3 along the axis, the corner of
// its end rounded to 0.5.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (slides)

in std.top {
  a := point
  b := point hint((8, -8))
  c := point hint((8, 8))
  d := point hint((-8, 8))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-8, -8)) a
  a distance(16) b
  b distance(16) c
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
fix(origin == (0, 0, 4)) bench
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

down := axis hint(dir: (0, 0, -1))
down parallel(sense: opposite) std.z
ram := frame
ram slides(8mm) down
hole := solid({ p | some t in (0deg, 360deg): p in mill at(under: ram) t })
hole cut part
