// A torus shrunk by 1.5: the tube's circle, radius 3, drawn in the front plane (x, z) 10 out along
// x and turned a whole turn about the z axis; and the points of it at least 1.5 from its outside,
// `std.rounds.Grown`'s body for a negative reach.
unit mm
use std
use std.solids
use std.rounds

in std.front {
  c := point
  fix((10, 0)) c
  tube := circle(center: c) hint(r: 3)
  radius(3) tube
}

torus := solid(std.solids.Revolution(face(tube), about: std.z))
shrunk := std.rounds.Grown(torus, d: -1.5mm)
part := solid(shrunk.body)
