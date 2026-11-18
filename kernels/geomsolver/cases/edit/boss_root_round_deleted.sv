// A plate, the circle of radius 8 about the origin drawn in the top plane and its prism from
// z = −4 to z = 0, with a boss standing on it, the circle of radius 3 about the same centre and
// its prism from z = 0 to z = 8, joined to it; the concave edge where the boss's faces meet the
// plate's top (`near`) rounded to radius 1, a fill. Then the round deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is bounded by the surfaces of every face of the bossed plate but
// the round's, read from its own faces. It is not on one side of all of them: it is on the
// material's side of the plate's bottom and rim and of the boss's top, and of the plate's top or
// the boss's side.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds (round)

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
plate.boss round(r: 1mm) plate.near

part := solid({ p |
  p in plate.far.side
  p in plate.rim.side
  p in plate.boss.near.side
  p in plate.near.side or p in plate.boss.stud.side
})
