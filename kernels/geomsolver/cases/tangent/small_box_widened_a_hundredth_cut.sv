// A small square prism, 0.0015 square about the z axis, and a copy a hundredth wider, 0.001515
// square about the same axis: each square drawn about the origin of the top plane (x, y), and the
// prism over it from z = 0 to z = 0.0015. The copy less the prism.
unit mm
use std
use std.shapes
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  narrow := std.shapes.CenteredRectangle(o, w: 0.0015mm, h: 0.0015mm)
  wide := std.shapes.CenteredRectangle(o, w: 0.001515mm, h: 0.001515mm)
}

prism := solid(std.solids.Prism(narrow.loop, from: 0mm, to: 0.0015mm))
part := solid(std.solids.Prism(wide.loop, from: 0mm, to: 0.0015mm))
prism cut part
