// An elliptic cylinder: the ellipse of semi-axes 5 along x and 3 along y about the origin of the
// top plane (x, y), a whole turn of `std.shapes.Ellipse` traced, and the prism over it from z = 0
// to z = 10.
unit mm
use std
use std.shapes
use std.solids

in std.top {
  o := point
  fix((0, 0)) o
}
rim := trace(std.shapes.Ellipse(o, a: 5mm, b: 3mm, tilt: 0deg).p over u in (0deg, 360deg))

part := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
