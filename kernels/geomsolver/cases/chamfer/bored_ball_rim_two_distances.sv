// A ball of radius 5 about the origin bored right through along the z axis by a hole of radius 2,
// the upper rim where the bore meets the ball chamfered 2 down the bore and 1 on the ball, each
// measured in a straight line from the rim in the plane through the axis: the chamfer is the cone
// between the circle 2 down the bore and the circle on the ball whose points are a chord of 1 from
// the rim. The lower rim stays sharp.
//
// The library's chamfer is not that cone: its flat is where the distances from the two surfaces,
// each over the other's leg, sum to one, which bows beside a ball, and its legs are distances from
// the other surface, not chords from the edge. So the chamfered section is drawn in the front plane
// (x the radius, z up) with the chamfer as one of its sides, and turned about the z axis: the
// ball's arc from the lower rim `low` up to `q`, a chord of 1 from the upper rim `e`; the chamfer
// to `foot`, 2 down the bore from the rim; down the bore to the lower rim.
unit mm
use std
use std.solids

in std.front {
  o := point
  fix((0, 0)) o
  e := point hint((2, 4.6))
  low := point hint((2, -4.6))
  o distance(5mm) e
  o distance(2mm, along: u) e
  o distance(2mm, along: u) low
  foot := point hint((2, 2.6))
  q := point hint((2.9, 4.1))
  foot distance(2mm, along: v) e
  e distance(1mm) q
  section := (ball := arc(center: o, start: low, end: q)) -> (chamfer := line(q, foot)) ->
             vertical (bore := line(foot, low)) -> close
  radius(5mm) ball
}

part := solid(std.solids.Revolution(section, about: std.z))
