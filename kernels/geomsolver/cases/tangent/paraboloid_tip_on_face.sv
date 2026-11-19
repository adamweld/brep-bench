// A block from (-10, -10, 0) to (10, 10, 10), its square drawn about the origin of the top plane
// (x, y) and the prism over it, united with a solid paraboloid about the z axis, its vertex at the
// origin, the middle of the block's bottom face, filled to z = 12.5. The paraboloid is its
// half-section turned a whole turn about z: in the front plane (x, z), the parabola x² = 8z traced
// from the vertex out to where it reaches z = 12.5, the level line back to the axis, and the axis
// down to the vertex.
unit mm
use std
use std.shapes
use std.solids
use std.solids (union)
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
  centre := point hint((0, 12.5))
  o distance(12.5mm, along: v) centre
  o vertical centre
  rim := point hint((10, 12.5))
  rim coincident side
  rim horizontal centre
  lid := line(rim, centre)
  spine := line(centre, o)
  o coincident side
  half := face(side from o to rim, lid, spine)
}

side := trace(Parabola(o).p over u in (0mm, 11mm))

paraboloid := solid(std.solids.Revolution(half, about: std.z))
part := solid(std.solids.Prism(square.loop, from: 0mm, to: 10mm))
paraboloid union part
