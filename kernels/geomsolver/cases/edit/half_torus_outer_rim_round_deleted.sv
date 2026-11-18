// The half of the ring torus about the z axis above the top plane: the upper half of the circle
// of radius 2 about (6, 0), drawn in the front plane (x, z) as two quarter arcs meeting tangent at
// its top, the outer and the inner, closed by its diameter, turned a whole turn about the z axis.
// Both rims lie between the flat and the torus, so the torus is drawn as two faces to name the
// outer rim alone: where the flat meets the outer face, at radius 8, rounded to radius 1/2. Then
// the round deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face of
// the rounded half torus but the round's (its flat and the torus's two), read from its own faces.
unit mm
use std
use std.solids
use std.place (horizontal, vertical)
use std.rounds (round)

in std.front {
  k := point
  fix((6, 0)) k
  s := point hint((8, 0))
  m := point hint((6, 2))
  e := point hint((4, 0))
  s horizontal k
  m vertical k
  e horizontal k
  outer := arc(center: k, start: s, end: m) hint(r: 2)
  inner := arc(center: k, start: m, end: e) hint(r: 2)
  radius(2) outer
  flat := line(e, s)
}

ring := solid(std.solids.Revolution(face(outer, inner, flat), about: std.z))
ring.flat round(r: 0.5mm) ring.outer

part := solid({ p | p in ring.outer.side; p in ring.inner.side; p in ring.flat.side })
