// A rounded puck shrunk by 2: a cylinder of radius 6 on the z axis from z = 0 to z = 10 (its circle
// drawn in the top plane, the prism over it), both its circular edges rounded to radius 2, and the
// points of it at least 2 from its outside, `std.rounds.Grown`'s body for a negative reach.
unit mm
use std
use std.solids
use std.rounds
use std.rounds (round)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 6)
  radius(6) rim
}

puck := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
round(r: 2mm) puck
shrunk := std.rounds.Grown(puck, d: -2mm)
part := solid(shrunk.body)
