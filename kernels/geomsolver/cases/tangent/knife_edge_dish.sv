// A cylinder of radius 5 on the z axis, its circle drawn about the origin of the top plane (x, y)
// and the prism over it from z = 0 to z = 10, less the ball of radius 5 about the middle of its top
// face, 10 up the z axis: the ball's equator is the top face's rim.
unit mm
use std
use std.sets
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim
}

top := point
fix((0, 0, 10)) top

dish := solid(std.sets.Ball(top, r: 5mm))
part := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
dish cut part
