// A tube: the circle of radius 10 about the origin drawn in the top plane, its bore the circle of
// radius 4 about the same centre a hole in it, and the prism of that face from z = 0 to z = 20.
// A ring groove is turned into the bore: the ring from inside the bore out to radius 6, from
// z = 8 to z = 12, its section drawn in the front plane (x, z) and turned about the z axis, cut
// away. Then the groove's floor and both its walls deleted together.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the groove runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face of
// the grooved tube but the groove's (its two ends, its outside and its bore, below the groove and
// above it), read from its own faces.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 10)
  radius(10) rim
  bore := circle(center: o) hint(r: 4)
  radius(4) bore
}

tube := solid(std.solids.Prism(face(rim, holes: bore), from: 0mm, to: 20mm))

in std.front {
  g0 := point hint((3, 8))
  g1 := point hint((6, 8))
  g2 := point hint((6, 12))
  g3 := point hint((3, 12))
  horizontal (lower := line(g0, g1)) -> vertical (floor := line(g1, g2)) ->
  horizontal (upper := line(g2, g3)) -> vertical (inner := line(g3, g0)) -> close
  fix((3, 8)) g0
  g0 distance(3) g1
  g1 distance(4) g2
}
groove := solid(std.solids.Revolution(face(lower, floor, upper, inner), about: std.z))
groove cut tube

part := solid({ p |
  p in tube.near.side
  p in tube.far.side
  p in tube.rim.side
  p in tube.bore.side
})
