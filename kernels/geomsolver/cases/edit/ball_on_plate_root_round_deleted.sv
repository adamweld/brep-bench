// A plate, the circle of radius 8 about the origin drawn in the top plane and its prism from
// z = −2 to z = 0, and the ball of radius 3 about (0, 0, 1) joined to it, standing up out of the
// plate's top; the concave edge where the ball meets the plate's top (`near`) rounded to radius 1,
// a fill. Then the round deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is bounded by the surfaces of every face of the solid but the
// round's, read from its own faces: on the material's side of the plate's bottom and rim, and of
// the plate's top or the ball.
unit mm
use std
use std.sets
use std.solids
use std.solids (union)
use std.rounds (round)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 8)
  radius(8) rim
}
c := point
fix((0, 0, 1)) c

plate := solid(std.solids.Prism(face(rim), from: -2mm, to: 0mm))
globe := std.sets.Ball(c, r: 3mm)
ball := solid({ p | p in globe })
ball union plate
plate.ball round(r: 1mm) plate.near

part := solid({ p |
  p in plate.far.side
  p in plate.rim.side
  p in plate.near.side or p in plate.ball.globe.side
})
