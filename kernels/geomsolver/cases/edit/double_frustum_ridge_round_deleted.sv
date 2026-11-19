// Two cone frustums on the z axis made as one solid, a turn of their half-section drawn in the
// front plane (x, z) about the z axis: the lower cone (swept from `lower`) widening from radius 3
// at z = 0 to radius 5 at z = 4, the upper (from `upper`) narrowing from there to radius 2 at
// z = 8. Their ridge, where the two cones meet, rounded to radius 1. Then the round deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face of
// the rounded solid but the round's (its base, its two cones and its top), read from its own
// faces.
unit mm
use std
use std.solids
use std.rounds (round)

in std.front {
  a := point
  b := point hint((3, 0))
  c := point hint((5, 4))
  d := point hint((2, 8))
  e := point hint((0, 8))
  horizontal (base := line(a, b)) -> (lower := line(b, c)) -> (upper := line(c, d)) ->
  horizontal (top := line(d, e)) -> vertical (core := line(e, a)) -> close
  fix((0, 0)) a
  a distance(3) b
  fix((5, 4)) c
  a distance(8) e
  e distance(2) d
}

pair := solid(std.solids.Revolution(face(base, lower, upper, top, core), about: std.z))
pair.lower round(r: 1mm) pair.upper

part := solid({ p |
  p in pair.base.side
  p in pair.lower.side
  p in pair.upper.side
  p in pair.top.side
})
