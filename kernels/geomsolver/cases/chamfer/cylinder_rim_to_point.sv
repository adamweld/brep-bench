// A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circle round its top face (`near`)
// chamfered 5 on both faces, the cylinder's own radius: the leg across the top reaches the axis, so
// the top shrinks to its centre and the flat is the cone from the circle of radius 5 at z = 5 up to
// its apex at (0, 0, 10).
unit mm
use std
use std.solids
use std.rounds (chamfer)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim
}

part := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
part.near chamfer(d: 5mm) part.rim
