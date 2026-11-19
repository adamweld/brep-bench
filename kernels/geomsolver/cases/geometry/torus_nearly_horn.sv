// A torus whose hole is nearly closed: the tube's circle, radius 4.99, drawn in the front plane
// (x, z) 5 out along x, turned a whole turn about the z axis, so the hole is 0.01 across.
unit mm
use std
use std.solids

in std.front {
  c := point
  fix((5, 0)) c
  tube := circle(center: c) hint(r: 4.99)
  radius(4.99mm) tube
}

part := solid(std.solids.Revolution(face(tube), about: std.z))
