// A stepped shaft on the z axis: the cylinder of radius 5 from z = 0 to z = 10, and the cylinder
// of radius 3 from z = 10 to z = 20 standing on it, in union. Both circles are drawn about the
// origin of the top plane.
unit mm
use std
use std.solids
use std.solids (union)

in std.top {
  o := point
  fix((0, 0)) o
  shoulder := circle(center: o) hint(r: 5)
  radius(5) shoulder
  journal := circle(center: o) hint(r: 3)
  radius(3) journal
}

upper := solid(std.solids.Prism(face(journal), from: 10mm, to: 20mm))
part := solid(std.solids.Prism(face(shoulder), from: 0mm, to: 10mm))
upper union part
