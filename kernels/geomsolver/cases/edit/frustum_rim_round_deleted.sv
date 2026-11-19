// A cone frustum on the z axis, a turn of its half-section drawn in the front plane (x, z) about
// the z axis: from radius 6 at z = 0 to radius 3 at z = 6. The rim where its top meets its cone
// (swept from `top` and `side`) is rounded to radius 1. Then the round deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the rounded frustum but the round's (its base, its cone and its top), read from its own
// faces.
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
frustum.top round(r: 1mm) frustum.side

part := solid({ p | p in frustum.base.side; p in frustum.side.side; p in frustum.top.side })
