// A cone frustum on the z axis, a turn of its half-section drawn in the front plane (x, z) about
// the z axis: from radius 6 at z = 0 to radius 3 at z = 6. Both rims, where its top and its base
// (swept from `top` and `base`) meet its cone (from `side`), are rounded to radius 3/2, each round
// named. Then the top's round deleted, the base's kept.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet. The faces left are the base, the cone, the top and the base's round, and the round
// bounds the solid only in part: what is left is the points on the material's side of the base's,
// the cone's and the top's surfaces that are in the rounded frustum or nearer the deleted round
// than the kept one.
unit mm
use std
use std.solids
use std.rounds (round)

in std.front {
  a := point
  b := point hint((6, 0))
  c := point hint((3, 6))
  d := point hint((0, 6))
  horizontal (base := line(a, b)) -> (side := line(b, c)) -> horizontal (top := line(c, d)) ->
  vertical (core := line(d, a)) -> close
  fix((0, 0)) a
  a distance(6) b
  a distance(6) d
  d distance(3) c
}

frustum := solid(std.solids.Revolution(face(base, side, top, core), about: std.z))
upper := frustum.top round(r: 1.5mm) frustum.side
lower := frustum.base round(r: 1.5mm) frustum.side

nearer_upper := { q | distance(q, frustum.upper) <= distance(q, frustum.lower) }
part := solid({ p |
  p in frustum.base.side
  p in frustum.side.side
  p in frustum.top.side
  p in frustum or p in nearer_upper
})
