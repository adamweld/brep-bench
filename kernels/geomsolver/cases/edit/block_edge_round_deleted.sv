// A block, its 20 × 10 footprint drawn in the top plane (x, y) from the origin and the prism over
// it to z = 10, the edge where its top (`near`) meets its side x = 20 (swept from `east`) rounded
// to radius 2. Then the round deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the rounded block but the round's, read from the block's own faces.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  a := point
  b := point hint((20, 0))
  c := point hint((20, 10))
  d := point hint((0, 10))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
  horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  fix((0, 0)) a
  a distance(20) b
  b distance(10) c
}

block := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 10mm))
block.east round(r: 2mm) block.near

part := solid({ p |
  p in block.near.side
  p in block.far.side
  p in block.south.side
  p in block.east.side
  p in block.north.side
  p in block.west.side
})
