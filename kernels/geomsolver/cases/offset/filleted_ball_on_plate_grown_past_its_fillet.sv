// A ball standing in a plate, its root filleted, grown by 2: the plate the prism over a circle of
// radius 8 about the origin of the top plane, from z = −2 to z = 0; the ball of radius 3 about
// (0, 0, 1) in union with it; the concave edge where the ball meets the plate's top rounded to
// radius 1; and the points within 2 of all that, `std.rounds.Grown`'s body.
unit mm
use std
use std.sets
use std.solids
use std.solids (union)
use std.rounds
use std.rounds (round)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 8)
  radius(8) rim
}
centre := point
fix((0, 0, 1)) centre

body := solid(std.solids.Prism(face(rim), from: -2mm, to: 0mm))
ball := solid(std.sets.Ball(centre, r: 3mm))
ball union body
body.ball round(r: 1mm) body.near
grown := std.rounds.Grown(body, d: 2mm)
part := solid(grown.body)
