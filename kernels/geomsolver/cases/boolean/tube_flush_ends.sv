// A tube cut flush: the cylinder of radius 5 on the z axis from z = 0 to z = 10, and the cylinder
// of radius 3 on the same axis over exactly the same span cut from it, its ends lying in the
// first's. Both circles are drawn about the origin of the top plane.
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

core := solid(std.solids.Prism(face(inner), from: 0mm, to: 10mm))
part := solid(std.solids.Prism(face(outer), from: 0mm, to: 10mm))
core cut part
