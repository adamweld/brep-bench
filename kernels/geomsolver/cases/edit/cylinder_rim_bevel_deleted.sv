// A cylinder of radius 6 on the z axis from z = 0 to z = 10, the edge round its top (`near`)
// beveled 2 back on each face: a cone from radius 6 at z = 8 to radius 4 at z = 10. Then the
// bevel deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the bevel runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the beveled cylinder but the bevel (its top, its bottom and its side), read from its own
// faces.
unit mm
use std
use std.solids
use std.rounds (chamfer)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 6)
  radius(6) rim
}

post := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
post.near chamfer(d: 2mm) post

part := solid({ p | p in post.near.side; p in post.far.side; p in post.rim.side })
