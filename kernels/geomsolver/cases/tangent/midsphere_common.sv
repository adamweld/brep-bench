// A cube of side 10 about the origin, its middle section drawn about the origin of the front plane
// (x, z) and the prism over it from y = -5 to y = 5, and the ball about the origin that touches each
// of the cube's twelve edges at its middle. The ball's radius is not written: its sphere passes
// through a corner of the middle section, which is the middle of the edge the section is swept
// along, and the solve finds 5√2. Their common part.
unit mm
use std
use std.sets
use std.shapes
use std.solids
use std.solids (common)

side := 10mm

in std.front {
  o := point
  fix((0, 0)) o
  middle := std.shapes.CenteredRectangle(o, w: side, h: side)
}

ball := std.sets.Ball(o, r: hint(7mm))
middle.a in boundary(ball)

cube := solid(std.solids.Prism(middle.loop, from: -side / 2, to: side / 2))
part := solid(ball)
cube common part
