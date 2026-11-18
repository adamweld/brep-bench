// A cone frustum on the z axis, a turn of its half-section drawn in the front plane (x, z) about
// the z axis: from radius 6 at z = 0 to radius 3 at z = 20. A flat is milled along its side:
// everything with y < −4 cut away, by the prism through a rectangle drawn in the top plane from
// y = −4 out past the frustum. Then the flat deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the flat runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the milled frustum but the flat (its base, its cone and its top), read from its own faces.
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

in std.top {
  m0 := point hint((-8, -8))
  m1 := point hint((8, -8))
  m2 := point hint((8, -4))
  m3 := point hint((-8, -4))
  slab := horizontal line(m0, m1) -> vertical line(m1, m2) -> horizontal (flat := line(m2, m3)) ->
          vertical line(m3, m0) -> close
  fix((-8, -8)) m0
  m0 distance(16) m1
  m1 distance(4) m2
  mill := solid(std.solids.Through(slab))
}

frustum := solid(std.solids.Revolution(face(base, side, top, core), about: std.z))
mill cut frustum

part := solid({ p | p in frustum.base.side; p in frustum.side.side; p in frustum.top.side })
