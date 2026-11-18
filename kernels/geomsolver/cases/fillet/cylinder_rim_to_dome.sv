// A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circle round its top face
// (`near`) rounded to radius 5, the cylinder's own: the top shrinks to a point and the round is a
// hemisphere.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim
}

part := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
part.near round(r: 5mm) part.rim
