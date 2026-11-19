// A torus about the z axis, its tube's circle of radius 3 drawn in the front plane (x, z) 10 out
// along x and turned a whole turn about z, united with a drum of radius 13 on the z axis: its circle
// drawn about the origin of the top plane (x, y) and the prism over it from z = -1 to z = 1. The
// torus's outer equator touches the drum's wall from inside.
unit mm
use std
use std.solids
use std.solids (union)

in std.front {
  c := point
  fix((10, 0)) c
  tube := circle(center: c) hint(r: 3)
  radius(3) tube
}

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 13)
  radius(13) rim
}

torus := solid(std.solids.Revolution(face(tube), about: std.z))
part := solid(std.solids.Prism(face(rim), from: -1mm, to: 1mm))
torus union part
