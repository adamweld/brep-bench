// A plate, the circle of radius 8 about the origin drawn in the top plane and its prism from
// z = −4 to z = 0, with a boss standing on it, the circle of radius 3 about the same centre and
// its prism from z = 0 to z = 8, joined to it. Then the boss's side and its top deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the boss runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the bossed plate but the boss's (the plate's top, closing over the boss's foot, its bottom
// and its rim), read from its own faces.
unit mm
use std
use std.solids
use std.solids (union)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 8)
  radius(8) rim
  stud := circle(center: o) hint(r: 3)
  radius(3) stud
}

plate := solid(std.solids.Prism(face(rim), from: -4mm, to: 0mm))
boss := solid(std.solids.Prism(face(stud), from: 0mm, to: 8mm))
boss union plate

part := solid({ p |
  p in plate.near.side
  p in plate.far.side
  p in plate.rim.side
})
