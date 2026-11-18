// A block with the edge where its top, z = 10, meets its side x = 20 beveled 3 back on each face:
// the pentagon through (0, 0), (20, 0), (20, 7), (17, 10) and (0, 10) drawn in the front plane
// (x, z), the prism behind it to y = 10, its bevel the side swept from `bevel`. Then the bevel
// deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the bevel runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the beveled block but the bevel, read from the block's own faces.
unit mm
use std
use std.solids

in std.front {
  a := point
  b := point hint((20, 0))
  c := point hint((20, 7))
  d := point hint((17, 10))
  e := point hint((0, 10))
  horizontal (bottom := line(a, b)) -> vertical (side := line(b, c)) -> (bevel := line(c, d)) ->
  horizontal (top := line(d, e)) -> vertical (back := line(e, a)) -> close
  fix((0, 0)) a
  a distance(20) b
  b distance(7) c
  a distance(10) e
  e distance(17) d
}

block := solid(std.solids.Prism(face(bottom, side, bevel, top, back), from: -10mm, to: 0mm))

part := solid({ p |
  p in block.bottom.side
  p in block.side.side
  p in block.top.side
  p in block.back.side
  p in block.near.side
  p in block.far.side
})
