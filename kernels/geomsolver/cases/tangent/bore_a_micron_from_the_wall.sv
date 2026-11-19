// A cylinder of radius 5 on the z axis from z = 0 to z = 10, bored right through along z by a hole
// of radius 4 whose axis is 0.999999 out along x: both circles drawn in the top plane (x, y), the
// cylinder the prism over the first and the cutter the prism over the second from z = -1 to
// z = 11, past both ends. At (5, 0, z) the wall left is a millionth thick.
unit mm
use std
use std.solids
use std.solids (cut)
use std.place (horizontal)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim

  c := point hint((1, 0))
  o distance(0.999999mm, along: u) c
  o horizontal c
  hole := circle(center: c) hint(r: 4)
  radius(4) hole
}

drill := solid(std.solids.Prism(face(hole), from: -1mm, to: 11mm))
part := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
drill cut part
