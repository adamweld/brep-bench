// A block, its 20 × 10 footprint drawn in the top plane (x, y) from the origin and the prism over
// it to z = 10, the two long edges of its top (`near`) rounded to radius 2: where it meets the
// side y = 0 (swept from `south`) and the side y = 10 (from `north`). Then both rounds deleted
// together.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the rounds runs the faces round them on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the rounded block but the rounds', read from the block's own faces.
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
block.south round(r: 2mm) block.near
block.north round(r: 2mm) block.near

part := solid({ p |
  p in block.near.side
  p in block.far.side
  p in block.south.side
  p in block.east.side
  p in block.north.side
  p in block.west.side
})
