// A cone frustum on the z axis, a turn of its half-section drawn in the front plane (x, z) about
// the z axis: from radius 6 at z = 0 to radius 3 at z = 20. A ring groove is turned into it:
// everything outside radius 4 from z = 8 to z = 12 cut away, by the turn of the rectangle from
// radius 4 out past the frustum drawn in the front plane. Then the groove's floor and both its
// walls deleted together.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the groove runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the grooved frustum but the groove's (its base, its cone below the groove and above it, and
// its top), read from its own faces.
unit mm
use std
use std.solids
use std.solids (cut)

in std.front {
  a := point
  b := point hint((6, 0))
  c := point hint((3, 20))
  d := point hint((0, 20))
  horizontal (base := line(a, b)) -> (side := line(b, c)) -> horizontal (top := line(c, d)) ->
  vertical (core := line(d, a)) -> close
  fix((0, 0)) a
  a distance(6) b
  a distance(20) d
  d distance(3) c
}

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

frustum := solid(std.solids.Revolution(face(base, side, top, core), about: std.z))
groove cut frustum

part := solid({ p | p in frustum.base.side; p in frustum.side.side; p in frustum.top.side })
