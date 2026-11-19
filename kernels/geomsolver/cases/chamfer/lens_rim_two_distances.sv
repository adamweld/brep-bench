// The lens common to two balls of radius 5 about (0, 0, 3) and (0, 0, -3), its rim — the circle of
// radius 4 at z = 0 where its two faces meet — chamfered 2 on the upper face (of the ball about
// (0, 0, -3)) and 1 on the lower, each measured in a straight line from the rim in the plane
// through the axis: the chamfer is the cone between the circles on the two faces whose points are
// those chords from the rim.
//
// The library's chamfer is not that cone: its flat is where the distances from the two surfaces,
// each over the other's leg, sum to one, which bows beside a ball, and its legs are distances from
// the other surface, not chords from the edge. So the chamfered half-section is drawn in the front
// plane (x the radius, z up) with the chamfer as one of its sides, and turned about the z axis:
// the lower face's arc (about `high`) from the lens's bottom up to `q1`, a chord of 1 from the
// rim `e`; the chamfer to `q2`, on the upper face a chord of 2 from the rim; the upper face's arc
// (about `low`) to the top; down the axis.
unit mm
use std
use std.solids

in std.front {
  high := point
  low := point
  fix((0, 3)) high
  fix((0, -3)) low
  e := point hint((4, 0))
  high distance(5mm) e
  low distance(5mm) e
  bottom := point hint((0, -2))
  top := point hint((0, 2))
  low distance(0mm, along: u) top
  q1 := point hint((3.3, -0.7))
  q2 := point hint((2.5, 1.3))
  e distance(1mm) q1
  e distance(2mm) q2
  half := (lower := arc(center: high, start: bottom, end: q1)) -> (chamfer := line(q1, q2)) ->
          (upper := arc(center: low, start: q2, end: top)) -> vertical line(top, bottom) -> close
  radius(5mm) lower
  radius(5mm) upper
}

part := solid(std.solids.Revolution(half, about: std.z))
