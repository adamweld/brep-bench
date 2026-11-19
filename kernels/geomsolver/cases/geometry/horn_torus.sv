// A horn torus: the tube's circle, radius 5, drawn in the front plane (x, z) 5 out along x, so it
// touches the z axis at the origin, turned a whole turn about the z axis.
unit mm
use std
use std.solids

in std.front {
  c := point
  fix((5, 0)) c
  tube := circle(center: c) hint(r: 5)
  radius(5mm) tube
}

part := solid(std.solids.Revolution(face(tube), about: std.z))
