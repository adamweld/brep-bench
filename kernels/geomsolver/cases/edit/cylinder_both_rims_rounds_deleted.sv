// A cylinder of radius 6 on the z axis from z = 0 to z = 10, both its rims rounded to radius 2:
// every edge among its faces. Then both rounds deleted together.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the rounds runs the faces round them on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the rounded cylinder but the rounds' (its top, its bottom and its side), read from its own
// faces.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 6)
  radius(6) rim
}

post := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
round(r: 2mm) post

part := solid({ p | p in post.near.side; p in post.far.side; p in post.rim.side })
