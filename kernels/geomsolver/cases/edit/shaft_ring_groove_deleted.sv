// A shaft, the circle of radius 5 about the origin drawn in the top plane and its prism from
// z = 0 to z = 20, with a ring groove turned into it: the ring from radius 4 out past the shaft,
// from z = 8 to z = 12, cut away. Then the groove's floor and both its walls deleted together.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit is
// said as one: deleting the groove runs the faces round it on along their own surfaces until they
// meet, so what is left is the points on the material's side of the surface of every face of the
// grooved shaft but the groove's (its two ends and its cylinder, below the groove and above it),
// read from its own faces.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim
}
shaft := solid(std.solids.Prism(face(rim), from: 0mm, to: 20mm))

// the groove's cutter, a turn of its section drawn in the front plane (x, z) about the z axis
in std.front {
  g0 := point hint((4, 8))
  g1 := point hint((7, 8))
  g2 := point hint((7, 12))
  g3 := point hint((4, 12))
  horizontal (lower := line(g0, g1)) -> vertical (outer := line(g1, g2)) ->
  horizontal (upper := line(g2, g3)) -> vertical (floor := line(g3, g0)) -> close
  fix((4, 8)) g0
  g0 distance(3) g1
  g1 distance(4) g2
}
groove := solid(std.solids.Revolution(face(lower, outer, upper, floor), about: std.z))
groove cut shaft

part := solid({ p | p in shaft.near.side; p in shaft.far.side; p in shaft.rim.side })
