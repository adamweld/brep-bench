// Two coaxial tori about the z axis, each a tube's circle of radius 2 drawn in the front plane
// (x, z) 5 out along x — the first's 1 below the x axis, the second's 1 above it — turned a whole
// turn about z. The part is the first kept where it is in common with the second.
unit mm
use std
use std.solids
use std.solids (common)
use std.place (offset)

in std.front {
  c1 := point hint((5, -1))
  c1 offset(dx: 5mm, dy: -1mm) std.origin
  tube1 := circle(center: c1) hint(r: 2)
  radius(2mm) tube1
  c2 := point hint((5, 1))
  c2 offset(dx: 5mm, dy: 1mm) std.origin
  tube2 := circle(center: c2) hint(r: 2)
  radius(2mm) tube2
}

second := solid(std.solids.Revolution(face(tube2), about: std.z))
part := solid(std.solids.Revolution(face(tube1), about: std.z))
second common part
