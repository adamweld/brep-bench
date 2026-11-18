// The part of a ball of radius 5 about the origin below the plane z = 3, its rim — the circle of
// radius 4 where the flat meets the ball — chamfered 2 on the flat face and 1 on the ball, each
// measured in a straight line from the rim in the plane through the axis: the chamfer is the cone
// between the circle 2 in from the rim on the flat and the circle on the ball whose points are a
// chord of 1 from the rim.
//
// The library's chamfer is not that cone: its flat is where the distances from the two surfaces,
// each over the other's leg, sum to one, which bows beside a ball, and its legs are distances from
// the other surface, not chords from the edge. So the chamfered half-section is drawn in the front
// plane (x the radius, z up) with the chamfer as one of its sides, and turned about the z axis:
// the ball's arc from its lower pole up to `q`, a chord of 1 below the rim `e`; the chamfer to
// `foot`, on the flat 2 in from the rim; along the flat to its centre; down the axis.
unit mm
use std
use std.solids

in std.front {
  o := point
  fix((0, 0)) o
  e := point hint((4, 3))
  o distance(5mm) e
  o distance(3mm, along: v) e
  centre := point hint((0, 3))
  o distance(0mm, along: u) centre
  o distance(3mm, along: v) centre
  pole := point hint((0, -5))
  foot := point hint((2, 3))
  q := point hint((4.5, 2.1))
  foot distance(2mm, along: u) e
  e distance(1mm) q
  half := (ball := arc(center: o, start: pole, end: q)) -> (chamfer := line(q, foot)) ->
          horizontal (flat := line(foot, centre)) -> vertical line(centre, pole) -> close
  radius(5mm) ball
}

part := solid(std.solids.Revolution(half, about: std.z))
