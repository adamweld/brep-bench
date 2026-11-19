// A cone of base radius 5 at z = 0, its apex 10 above on the z axis, less a cone of base radius
// 4.999 with the same base plane and apex: each the half-section, a right triangle drawn in the
// front plane (x, z) from the axis, turned a whole turn about the z axis.
unit mm
use std
use std.solids
use std.solids (cut)

in std.front {
  o := point
  apex := point hint((0, 10))
  rim := point hint((5, 0))
  inner_rim := point hint((4.999, 0))
  fix((0, 0)) o
  o distance(10) apex
  o distance(5) rim
  o distance(4.999) inner_rim

  cone := horizontal line(o, rim) -> line(rim, apex) -> vertical line(apex, o) -> close
  hollow := horizontal line(o, inner_rim) -> line(inner_rim, apex) -> line(apex, o) -> close
}

inside := solid(std.solids.Revolution(hollow, about: std.z))
part := solid(std.solids.Revolution(cone, about: std.z))
inside cut part
