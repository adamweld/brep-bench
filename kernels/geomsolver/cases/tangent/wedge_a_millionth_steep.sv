// A cube of side 10 from (-5, -5, -10) to (5, 5, 0), its square drawn about the origin of the top
// plane (x, y) and the prism over it from z = -10 to z = 0, in common with the half-space above the
// plane z = 10^-6 y. The half-space is a block over that plane: its section drawn in the side plane
// (y, z), one side along the plane's trace through the origin, rising 10^-6 in z for every 1 in y,
// and the square stood on it toward +z; the prism over it from x = -6 to x = 6 reaches past the
// cube.
unit mm
use std
use std.shapes
use std.solids
use std.solids (common)

in std.top {
  m := point
  fix((0, 0)) m
  square := std.shapes.CenteredRectangle(m, w: 10mm, h: 10mm)
}

in std.side {
  o := point
  fix((0, 0)) o
  a := point hint((-6, 0))
  b := point hint((6, 0))
  c := point hint((6, 12))
  d := point hint((-6, 12))
  block := (sole := line(a, b)) -> (rise := line(b, c)) -> (over := line(c, d)) ->
           (fall := line(d, a)) -> close
  o distance(6, along: u) b
  o distance(0.000006, along: v) b
  o midpoint sole
  sole perpendicular(sense: ccw) rise
  rise perpendicular(sense: ccw) over
  rise equal sole
  over equal sole
}

above := solid(std.solids.Prism(block, from: -6mm, to: 6mm))
part := solid(std.solids.Prism(square.loop, from: -10mm, to: 0mm))
above common part
