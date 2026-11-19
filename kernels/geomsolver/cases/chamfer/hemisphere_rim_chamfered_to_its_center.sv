// The upper half of a ball of radius 5 about the origin, its rim chamfered 5 on the flat face — the
// whole way to its centre — and 1 on the ball, each measured in a straight line from the rim in the
// plane through the axis: the flat face shrinks to the centre, and the chamfer is the cone from it
// up to the circle on the ball whose points are a chord of 1 from the rim.
//
// The library's chamfer is not that cone: its flat is where the distances from the two surfaces,
// each over the other's leg, sum to one, which bows beside a ball, and its legs are distances from
// the other surface, not chords from the edge. So the chamfered half-section is drawn in the front
// plane (x the radius, z up) with the chamfer as one of its sides, and turned about the z axis:
// from the centre, 5 in from the rim `e`, the chamfer to `q`, on the ball a chord of 1 from the
// rim; the ball's arc up to the pole; down the axis.
unit mm
use std
use std.solids

in std.front {
  o := point
  fix((0, 0)) o
  e := point hint((5, 0))
  q := point hint((4.9, 1))
  pole := point hint((0, 5))
  o distance(5mm, along: u) e
  o distance(0mm, along: v) e
  e distance(1mm) q
  half := (chamfer := line(o, q)) -> (dome := arc(center: o, start: q, end: pole)) ->
          vertical line(pole, o) -> close
  radius(5mm) dome
}

part := solid(std.solids.Revolution(half, about: std.z))
