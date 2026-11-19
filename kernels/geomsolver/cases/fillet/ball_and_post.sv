// A ball of radius 10 about the origin and a post, the cylinder of radius 4 on the z axis from
// z = 0 to z = 15, in union with it; the circle where the post's faces leave the ball's is
// rounded to radius 2, filled.
unit mm
use std
use std.solids
use std.solids (union)
use std.sets
use std.rounds (round)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 4)
  radius(4mm) rim
}

post := solid(std.solids.Prism(face(rim), from: 0mm, to: 15mm))
ball := solid(std.sets.Ball(std.origin, r: 10mm))
part := solid(ball)
post union part
part.post round(r: 2mm) part.ball
