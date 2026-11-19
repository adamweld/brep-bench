// A curved block, the ring sector between radii 8 and 12 about the z axis, z within 1, polar angle
// from 0 to 60 degrees: its section in the front plane (x, z) turned 60 degrees about the z axis.
// The bar is drawn in the plane of its flat end, through (10, 0, 0), its u = (cos 20, 0, -sin 20)
// (x turned 20 degrees about y) and v = y: its end the rectangle 3 deep and 4 wide whose inner side
// is bisected by the plane's origin, its sides turned 5 degrees from the plane's axes; the bar is
// that rectangle carried 3 along the plane's normal, leaning 20 degrees out from upright. It is
// turned rigidly about the z axis from -140 to +140 degrees, and everything it passes through cut
// from the block.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (turns)

in std.front {
  a := point
  b := point hint((12, -1))
  c := point hint((12, 1))
  d := point hint((8, 1))
  meridian := horizontal line(a, b) -> vertical line(b, c) ->
              horizontal line(c, d) -> vertical line(d, a) -> close
  fix((8, -1)) a
  a distance(4) b
  b distance(2) c
}
part := solid(std.solids.Revolution(meridian, about: std.z, to: 60deg))

across := axis hint(dir: (0.94, 0, -0.34))
across perpendicular std.y
std.x angle(20deg, about: std.y) across
along := axis hint(dir: (0, 1, 0))
along parallel(sense: same) std.y
holder := plane(u: across, v: along)
fix(origin == (10, 0, 0)) holder
in holder {
  o := point
  fix((0, 0)) o
  b0 := point hint((0.17, -2))
  b1 := point hint((3.16, -1.73))
  b2 := point hint((2.81, 2.25))
  b3 := point hint((-0.17, 2))
  end := (lower := line(b0, b1)) -> perpendicular (outer_side := line(b1, b2)) -> perpendicular
         (upper := line(b2, b3)) -> perpendicular (inner_side := line(b3, b0)) -> close
  o midpoint inner_side
  length(4) inner_side
  length(3) lower
  holder.u angle(5deg) lower
}
bar := solid(std.solids.Prism(end, from: 0mm, to: 3mm))

swing := frame
swing turns(1) std.z
floor := solid({ p | some t in (-140deg, 140deg): p in bar at(under: swing) t })
floor cut part
