// A cylinder of radius 6 on the z axis from z = 0 to z = 10, the edge round its top (`near`)
// rounded to radius 2. Then the round deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the rounded cylinder but the round's (its top, its bottom and its side), read from its own
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
post.near round(r: 2mm) post

part := solid({ p | p in post.near.side; p in post.far.side; p in post.rim.side })
