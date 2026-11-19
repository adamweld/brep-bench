// The elliptic cylinder x²/25 + y²/9 ≤ 1 from z = 0 to z = 10, shrunk by 1: the ellipse of
// semi-axes 5 along x and 3 along y traced about the origin of the top plane (x, y)
// (`std.shapes.Ellipse`), the prism over it, and the points of it at least 1 from its outside,
// `std.rounds.Grown`'s body.
unit mm
use std
use std.solids
use std.shapes
use std.rounds

in std.top {
  o := point
  fix((0, 0)) o
}
rim := trace(std.shapes.Ellipse(o, a: 5mm, b: 3mm, tilt: 0deg).p over u in (0deg, 360deg))

cylinder := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
shrunk := std.rounds.Grown(cylinder, d: -1mm)
part := solid(shrunk.body)
