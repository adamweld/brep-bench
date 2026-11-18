// A paraboloid cup about the z axis, its vertex at the origin, filled to z = 5, united with the ball
// of radius 5 about (0, 0, 5). The cup is its half-section turned a whole turn about z: in the front
// plane (x, z), the parabola x² = 10z traced from the vertex out to where it reaches z = 5, the
// level line back to the axis, and the axis down to the vertex. The ball's bottom fits the bottom
// of the cup, where both meet the axis.
unit mm
use std
use std.sets
use std.solids
use std.solids (union)
use std.place (horizontal, vertical)

// a point of the parabola z = x² / 10 drawn from its vertex `o`, `u` out along x
component Parabola(o: point, u: Length) {
  p := point(x: o.x + u, y: o.y + u * u / 10mm)
}

in std.front {
  o := point
  fix((0, 0)) o
  centre := point hint((0, 5))
  o distance(5mm, along: v) centre
  o vertical centre
  rim := point hint((7, 5))
  rim coincident cup_side
  rim horizontal centre
  lid := line(rim, centre)
  spine := line(centre, o)
  o coincident cup_side
  half := face(cup_side from o to rim, lid, spine)
}

cup_side := trace(Parabola(o).p over u in (0mm, 8mm))

ball := solid(std.sets.Ball(centre, r: 5mm))
part := solid(std.solids.Revolution(half, about: std.z))
ball union part
