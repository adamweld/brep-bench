// A torus grown by 3: the tube's circle, radius 2, drawn in the front plane (x, z) 4 out along
// x and turned a whole turn about the z axis; and the points within 3 of it,
// `std.rounds.Grown`'s body. Grown past its hole, the tube closes over the axis.
unit mm
use std
use std.solids
use std.rounds

in std.front {
  c := point
  fix((4, 0)) c
  tube := circle(center: c) hint(r: 2)
  radius(2) tube
}

torus := solid(std.solids.Revolution(face(tube), about: std.z))
grown := std.rounds.Grown(torus, d: 3mm)
part := solid(grown.body)
