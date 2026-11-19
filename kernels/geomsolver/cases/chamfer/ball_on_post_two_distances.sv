// A post, the cylinder of radius 3 on the z axis from z = 0 to z = 10, joined to a ball of radius 5
// about (0, 0, 10); the post's side meets the ball in a concave edge. The edge chamfered 2 down the
// post and 1 on the ball, each measured in a straight line from the edge in the plane through the
// axis, adding material: the chamfer is the cone between the circle 2 down the post's side and the
// circle on the ball whose points are a chord of 1 from the edge.
//
// The library's chamfer is not that cone: its flat is where the distances from the two surfaces,
// each over the other's leg, sum to one, which bows beside a ball, and its legs are distances from
// the other surface, not chords from the edge. So the chamfered half-section is drawn in the front
// plane (x the radius, z up) with the chamfer as one of its sides, and turned about the z axis:
// across the post's foot; up its side to `foot`, 2 below the edge `e`; the chamfer to `q`, on the
// ball a chord of 1 from the edge; the ball's arc (about `k`) over the top to the pole; down the
// axis.
unit mm
use std
use std.solids

in std.front {
  o := point
  fix((0, 0)) o
  k := point
  fix((0, 10)) k
  b := point hint((3, 0))
  e := point hint((3, 6))
  foot := point hint((3, 4))
  q := point hint((3.7, 6.7))
  pole := point hint((0, 15))
  o distance(3mm) b
  o distance(3mm, along: u) e
  k distance(5mm) e
  foot distance(2mm, along: v) e
  e distance(1mm) q
  half := horizontal line(o, b) -> vertical (side := line(b, foot)) -> (chamfer := line(foot, q)) ->
          (ball := arc(center: k, start: q, end: pole)) -> vertical line(pole, o) -> close
  radius(5mm) ball
}

part := solid(std.solids.Revolution(half, about: std.z))
