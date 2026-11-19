// A cylinder: a circle of radius 5 about the origin of the top plane (x, y), and the prism over it
// from z = 0 to z = 10.
unit mm
use std
use std.solids

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim
}

part := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
