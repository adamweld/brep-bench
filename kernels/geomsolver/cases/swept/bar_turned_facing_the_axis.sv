// A curved block, the ring sector between radii 8 and 12 about the z axis, z within 1, polar angle
// from 0 to 60 degrees: its section in the front plane (x, z) turned 60 degrees about the z axis.
// The bar is drawn in a plane square to its length, through (10, 0, 0), its u = (cos 20, 0,
// -sin 20) (x turned 20 degrees about y, the way its inner side faces) and v = y: its section the
// rectangle from its inner side, the plane's v axis, 3 out along u and within 2 of the origin
// along v; the bar is that section carried 3 either way along the plane's normal, leaning 20
// degrees out from upright. It is turned rigidly about the z axis from -140 to +140 degrees, and
// everything it passes through cut from the block.
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
  b0 := point
  b1 := point hint((3, -2))
  b2 := point hint((3, 2))
  b3 := point hint((0, 2))
  section := horizontal line(b0, b1) -> vertical line(b1, b2) ->
             horizontal line(b2, b3) -> vertical line(b3, b0) -> close
  fix((0, -2)) b0
  b0 distance(3) b1
  b1 distance(4) b2
}
bar := solid(std.solids.Prism(section, from: -3mm, to: 3mm))

swing := frame
swing turns(1) std.z
wall := solid({ p | some t in (-140deg, 140deg): p in bar at(under: swing) t })
wall cut part
