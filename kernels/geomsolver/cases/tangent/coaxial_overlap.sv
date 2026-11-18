// Two cylinders of radius 5 on the z axis, both the prism over one circle about the origin of the
// top plane (x, y), the first from z = 0 to z = 10 and the second from z = 6 to z = 16, united.
unit mm
use std
use std.solids
use std.solids (union)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim
}

second := solid(std.solids.Prism(face(rim), from: 6mm, to: 16mm))
part := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
second union part
