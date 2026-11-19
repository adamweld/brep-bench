// The half of the ring torus about the z axis above the top plane: the upper half of the circle
// of radius 2 about (6, 0), drawn in the front plane (x, z) as an arc closed by its diameter,
// turned a whole turn about the z axis. Both its rims, where the flat meets the torus at radius 4
// and at radius 8, rounded to radius 1/2: every edge among its faces. Then both rounds deleted
// together.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the rounds runs the faces round them on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face of
// the rounded half torus but the rounds' (its flat and the torus), read from its own faces.
unit mm
use std
use std.solids
use std.place (horizontal)
use std.rounds (round)

in std.front {
  k := point
  fix((6, 0)) k
  s := point hint((8, 0))
  e := point hint((4, 0))
  s horizontal k
  e horizontal k
  tube := arc(center: k, start: s, end: e) hint(r: 2)
  radius(2) tube
  flat := line(e, s)
}

ring := solid(std.solids.Revolution(face(tube, flat), about: std.z))
round(r: 0.5mm) ring

part := solid({ p | p in ring.tube.side; p in ring.flat.side })
