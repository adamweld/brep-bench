// A small cylinder of radius 0.00075 and height 0.0015 on the z axis, and a copy a hundredth wider,
// of radius 0.0007575, on the same axis between the same two planes: both the prism from z = 0 to
// z = 0.0015 over a circle about the origin of the top plane (x, y). The copy less the cylinder.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 0.00075)
  radius(0.00075) rim
  wide := circle(center: o) hint(r: 0.0007575)
  radius(0.0007575) wide
}

cylinder := solid(std.solids.Prism(face(rim), from: 0mm, to: 0.0015mm))
part := solid(std.solids.Prism(face(wide), from: 0mm, to: 0.0015mm))
cylinder cut part
