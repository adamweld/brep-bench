// A ball of radius 5 about the origin, united with a cylinder of the same radius on the z axis: its
// circle drawn about the origin of the top plane (x, y), and the prism over it from z = -3 to z = 3.
unit mm
use std
use std.sets
use std.solids
use std.solids (union)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim
}

ball := solid(std.sets.Ball(o, r: 5mm))
part := solid(std.solids.Prism(face(rim), from: -3mm, to: 3mm))
ball union part
