// A frustum as a turn of its half-section: in the front plane (x, z), the quadrilateral from the
// axis out to radius 5 at z = 0 and radius 2 at z = 6, turned a whole turn about the z axis.
unit mm
use std
use std.solids

in std.front {
  a := point
  b := point hint((5, 0))
  c := point hint((2, 6))
  d := point hint((0, 6))
  half := horizontal line(a, b) -> line(b, c) -> horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(5) b
  a distance(6) d
  d distance(2) c
}

part := solid(std.solids.Revolution(half, about: std.z))
