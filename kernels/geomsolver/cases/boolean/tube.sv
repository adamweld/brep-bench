// A tube: the cylinder of radius 5 on the z axis from z = 0 to z = 10, and the bore, the cylinder
// of radius 3 on the same axis from z = −1 to z = 11, cut from it. Both circles are drawn about
// the origin of the top plane.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  outer := circle(center: o) hint(r: 5)
  radius(5) outer
  inner := circle(center: o) hint(r: 3)
  radius(3) inner
}

bore := solid(std.solids.Prism(face(inner), from: -1mm, to: 11mm))
part := solid(std.solids.Prism(face(outer), from: 0mm, to: 10mm))
bore cut part
