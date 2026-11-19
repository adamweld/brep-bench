// A solid torus about the z axis, its tube's circle of radius 2 drawn in the front plane (x, z) 6
// out along x and turned a whole turn, hollowed to a wall 0.5 thick with no opening: a closed
// cavity, the torus of tube radius 1.5, stands inside it.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.front {
  c := point
  fix((6, 0)) c
  tube := circle(center: c) hint(r: 2)
  radius(2) tube
}

part := solid(std.solids.Revolution(face(tube), about: std.z))
shell(t: 0.5mm) part
