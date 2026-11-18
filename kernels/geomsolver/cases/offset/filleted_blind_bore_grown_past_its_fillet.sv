// A block with a blind bore, filleted at the bore's floor, grown by 3: the block the prism over a
// circle of radius 10 about the origin of the top plane, from z = 0 to z = 8; the bore the prism
// over one of radius 5 from z = 2 up past the top, cut from it; the concave edge where the bore's
// side meets its floor rounded to radius 2; and the points within 3 of all that,
// `std.rounds.Grown`'s body.
unit mm
use std
use std.solids
use std.solids (cut)
use std.rounds
use std.rounds (round)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 10)
  radius(10) rim
  mouth := circle(center: o) hint(r: 5)
  radius(5) mouth
}

block := solid(std.solids.Prism(face(rim), from: 0mm, to: 8mm))
bore := solid(std.solids.Prism(face(mouth), from: 2mm, to: 12mm))
bore cut block
round(r: 2mm) block.bore
grown := std.rounds.Grown(block, d: 3mm)
part := solid(grown.body)
