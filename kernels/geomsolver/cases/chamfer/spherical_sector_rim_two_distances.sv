// A spherical sector: the part of a ball of radius 5 about the origin inside the cone about the z
// axis with its apex at the origin and half angle α, cos α = 3/5, whose side meets the ball in the
// sector's rim. The rim chamfered 2 down the cone and 1 on the ball, each measured in a straight
// line from the rim in the plane through the axis: the chamfer is the cone between the circle 2
// down the side and the circle on the ball whose points are a chord of 1 from the rim.
//
// The library's chamfer is not that cone: its flat is where the distances from the two surfaces,
// each over the other's leg, sum to one, which bows beside a ball, and its legs are distances from
// the other surface, not chords from the edge. So the chamfered half-section is drawn in the front
// plane (x the radius, z up) with the chamfer as one of its sides, and turned about the z axis: up
// the cone's side from the apex to `foot`, 2 short of the rim `e`; the chamfer to `q`, on the ball
// a chord of 1 from the rim; the ball's arc up to the pole; down the axis.
unit mm
use std
use std.solids

in std.front {
  o := point
  fix((0, 0)) o
  pole := point hint((0, 5))
  construction up := line(o, pole)
  e := point hint((4, 3))
  o distance(5mm) e
  foot := point hint((2.4, 1.8))
  q := point hint((3.3, 3.7))
  half := (side := line(o, foot)) -> (chamfer := line(foot, q)) ->
          (dome := arc(center: o, start: q, end: pole)) -> vertical line(pole, o) -> close
  side angle(acos(3 / 5)) up
  e coincident side
  foot distance(2mm) e
  e distance(1mm) q
  radius(5mm) dome
}

part := solid(std.solids.Revolution(half, about: std.z))
