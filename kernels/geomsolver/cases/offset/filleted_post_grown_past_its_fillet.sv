// A post on a plate, filleted at its foot, grown by 3: the plate the prism over a circle of radius
// 8 about the origin of the top plane, from z = −4 to z = 0; the post the prism over one of radius
// 3, from z = 0 to z = 8, in union with it; the concave edge where the post's side meets the
// plate's top rounded to radius 2; and the points within 3 of all that, `std.rounds.Grown`'s body.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds
use std.rounds (round)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 8)
  radius(8) rim
  foot := circle(center: o) hint(r: 3)
  radius(3) foot
}

body := solid(std.solids.Prism(face(rim), from: -4mm, to: 0mm))
post := solid(std.solids.Prism(face(foot), from: 0mm, to: 8mm))
post union body
body.post round(r: 2mm) body.near
grown := std.rounds.Grown(body, d: 3mm)
part := solid(grown.body)
