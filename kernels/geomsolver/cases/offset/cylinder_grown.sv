// A cylinder of radius 5 on the z axis from z = 0 to z = 10, grown by 1: the circle drawn in the
// top plane (x, y), the prism over it, and the points within 1 of it, `std.rounds.Grown`'s body.
unit mm
use std
use std.solids
use std.rounds

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim
}

post := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
grown := std.rounds.Grown(post, d: 1mm)
part := solid(grown.body)
