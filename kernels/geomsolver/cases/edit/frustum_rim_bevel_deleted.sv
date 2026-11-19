// A cone frustum on the z axis with its top rim beveled, a turn of its half-section drawn in the
// front plane (x, z) about the z axis: the base out to radius 6 at z = 0, the side, the cone
// 2r + z = 12, up to radius 3.5 at z = 5, the bevel, a steeper cone, from there to the top, the
// disc of radius 2 at z = 6. Then the bevel deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the bevel runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the beveled frustum but the bevel (its base, its side and its top), read from its own faces.
unit mm
use std
use std.solids

in std.front {
  a := point
  b := point hint((6, 0))
  c := point hint((3.5, 5))
  d := point hint((2, 6))
  e := point hint((0, 6))
  horizontal (base := line(a, b)) -> (side := line(b, c)) -> (bevel := line(c, d)) ->
  horizontal (top := line(d, e)) -> vertical (core := line(e, a)) -> close
  fix((0, 0)) a
  a distance(6) b
  a distance(6) e
  e distance(2) d
  fix((3.5, 5)) c
}

frustum := solid(std.solids.Revolution(face(base, side, bevel, top, core), about: std.z))

part := solid({ p | p in frustum.base.side; p in frustum.side.side; p in frustum.top.side })
