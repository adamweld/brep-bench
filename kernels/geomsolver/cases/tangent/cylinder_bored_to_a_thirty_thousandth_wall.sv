// A cylinder of radius 5 and height 10 on the z axis, less one of radius 4.99997 on the same axis between
// the same two planes: both the prism from z = 0 to z = 10 over a circle about the origin of the top
// plane (x, y), and what is left a tube whose wall is 0.00003 thick.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  outer := circle(center: o) hint(r: 5)
  radius(5) outer
  inner := circle(center: o) hint(r: 4.99997)
  radius(4.99997) inner
}

bore := solid(std.solids.Prism(face(inner), from: 0mm, to: 10mm))
part := solid(std.solids.Prism(face(outer), from: 0mm, to: 10mm))
bore cut part
