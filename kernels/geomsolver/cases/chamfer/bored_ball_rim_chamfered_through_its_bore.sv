// A ball of radius 5 about the origin bored right through along the z axis by a hole of radius 2,
// the upper rim where the bore meets the ball chamfered 2√21 down the bore — its whole length, to
// the lower rim — and 1 on the ball, each measured in a straight line from the rim in the plane
// through the axis: the bore shrinks to its lower rim, and the chamfer is the cone from there up
// to the circle on the ball whose points are a chord of 1 from the upper rim.
//
// The library's chamfer is not that cone: its flat is where the distances from the two surfaces,
// each over the other's leg, sum to one, which bows beside a ball, and its legs are distances from
// the other surface, not chords from the edge. So the chamfered section is drawn in the front plane
// (x the radius, z up) with the chamfer as one of its sides, and turned about the z axis: the
// ball's arc from the lower rim `low` (2√21 down the bore from the upper rim `e`, both where the
// bore meets the ball) up to `q`, a chord of 1 from the upper rim; the chamfer back down to `low`.
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
  q := point hint((2.9, 4.1))
  e distance(1mm) q
  section := (ball := arc(center: o, start: low, end: q)) -> (chamfer := line(q, low)) -> close
  radius(5mm) ball
}

part := solid(std.solids.Revolution(section, about: std.z))
