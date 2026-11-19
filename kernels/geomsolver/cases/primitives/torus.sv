// A torus: the tube's circle, radius 2, drawn in the front plane (x, z) 10 out along x, turned a
// whole turn about the z axis.
unit mm
use std
use std.solids

in std.front {
  c := point
  fix((10, 0)) c
  tube := circle(center: c) hint(r: 2)
  radius(2) tube
}

part := solid(std.solids.Revolution(face(tube), about: std.z))
