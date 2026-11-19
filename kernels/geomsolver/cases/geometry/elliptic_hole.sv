// A block with an elliptic hole: the block -12 <= x <= 12, -8 <= y <= 8 drawn in the top plane
// and its prism from z = 0 to z = 8, with the ellipse of semi-axes 6 along x and 3 along y about
// the origin (a whole turn of `std.shapes.Ellipse` traced) cut through it as a prism through
// everything.
unit mm
use std
use std.shapes
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  a := point
  b := point hint((12, -8))
  c := point hint((12, 8))
  d := point hint((-12, 8))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-12, -8)) a
  a distance(24) b
  b distance(16) c
}
bore := trace(std.shapes.Ellipse(o, a: 6mm, b: 3mm, tilt: 0deg).p over u in (0deg, 360deg))

part := solid(std.solids.Prism(footprint, from: 0mm, to: 8mm))
drill := solid(std.solids.Through(face(bore)))
drill cut part
