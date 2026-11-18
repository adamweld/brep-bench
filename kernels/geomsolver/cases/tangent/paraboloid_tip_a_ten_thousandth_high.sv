// The tip of a solid paraboloid about the z axis, 10^-4 high: the paraboloid x² + y² <= 8z filled
// to z = 8, in common with a block whose top is the plane z = 10^-4. The paraboloid is its
// half-section turned a whole turn about z: in the front plane (x, z), the parabola x² = 8z traced
// from the vertex at the origin out to where it reaches z = 8, the level line back to the axis, and
// the axis down to the vertex. The block's square, 20 on a side, is drawn about the origin of the
// top plane (x, y), and the prism over it runs from z = -1 to z = 10^-4, past the paraboloid on
// every side but its top.
unit mm
use std
use std.shapes
use std.solids
use std.solids (common)
use std.place (horizontal, vertical)

// a point of the parabola z = x² / 8 drawn from its vertex `o`, `u` out along x
component Parabola(o: point, u: Length) {
  p := point(x: o.x + u, y: o.y + u * u / 8mm)
}

in std.top {
  m := point
  fix((0, 0)) m
  square := std.shapes.CenteredRectangle(m, w: 20mm, h: 20mm)
}

in std.front {
  o := point
  fix((0, 0)) o
  centre := point hint((0, 8))
  o distance(8mm, along: v) centre
  o vertical centre
  rim := point hint((8, 8))
  rim coincident side
  rim horizontal centre
  lid := line(rim, centre)
  spine := line(centre, o)
  o coincident side
  half := face(side from o to rim, lid, spine)
}

side := trace(Parabola(o).p over u in (0mm, 9mm))

below := solid(std.solids.Prism(square.loop, from: -1mm, to: 0.0001mm))
part := solid(std.solids.Revolution(half, about: std.z))
below common part
