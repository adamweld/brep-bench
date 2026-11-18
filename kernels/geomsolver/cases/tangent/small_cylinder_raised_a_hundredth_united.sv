// A small cylinder of radius 0.00075 and height 0.0015 on the z axis, the prism from z = 0 over a
// circle about the origin of the top plane (x, y), and a copy of it raised 0.000015 along z: the
// prism over the same circle from z = 0.000015 to z = 0.001515. Their union.
unit mm
use std
use std.solids
use std.solids (union)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 0.00075)
  radius(0.00075) rim
}

copy := solid(std.solids.Prism(face(rim), from: 0.000015mm, to: 0.001515mm))
part := solid(std.solids.Prism(face(rim), from: 0mm, to: 0.0015mm))
copy union part
