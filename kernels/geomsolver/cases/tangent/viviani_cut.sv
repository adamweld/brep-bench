// Viviani's solid: a ball of radius 10 about the origin, and a cylinder of radius 5 whose wall
// passes through the ball's centre and touches its sphere from inside at (10, 0, 0): its circle
// drawn in the top plane (x, y) about a centre 5 out along x, and the prism over it from z = -12 to
// z = 12. The ball less the cylinder.
unit mm
use std
use std.sets
use std.solids
use std.solids (cut)
use std.place (horizontal)

in std.top {
  o := point
  fix((0, 0)) o
  c := point hint((5, 0))
  o distance(5mm, along: u) c
  o horizontal c
  rim := circle(center: c) hint(r: 5)
  radius(5) rim
}

bore := solid(std.solids.Prism(face(rim), from: -12mm, to: 12mm))
part := solid(std.sets.Ball(o, r: 10mm))
bore cut part
