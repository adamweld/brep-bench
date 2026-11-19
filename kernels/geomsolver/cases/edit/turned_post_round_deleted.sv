// A turned post on the z axis, a turn of its half-section drawn in the front plane (x, z) about the
// z axis, made as one solid: the cylinder of radius 5 from z = 0 to z = 6 (swept from `wall`) and
// on it the cone from radius 5 at z = 6 to radius 2 at z = 10 (from `cone`). Its shoulder, where
// the wall meets the cone, rounded to radius 1. Then the round deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face of
// the rounded post but the round's (its base, its wall, its cone and its top), read from its own
// faces.
unit mm
use std
use std.solids
use std.rounds (round)

in std.front {
  a := point
  b := point hint((5, 0))
  c := point hint((5, 6))
  d := point hint((2, 10))
  e := point hint((0, 10))
  horizontal (base := line(a, b)) -> vertical (wall := line(b, c)) -> (cone := line(c, d)) ->
  horizontal (top := line(d, e)) -> vertical (core := line(e, a)) -> close
  fix((0, 0)) a
  a distance(5) b
  b distance(6) c
  a distance(10) e
  e distance(2) d
}

post := solid(std.solids.Revolution(face(base, wall, cone, top, core), about: std.z))
post.wall round(r: 1mm) post.cone

part := solid({ p |
  p in post.base.side
  p in post.wall.side
  p in post.cone.side
  p in post.top.side
})
