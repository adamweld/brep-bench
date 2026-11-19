// A block, its 20 × 10 footprint drawn in the top plane (x, y) from the origin and the prism over
// it to z = 10, with a slot cut into its side y = 0, top to bottom: the cutter's outline runs in
// from outside the block along x = 8 and x = 12 to y = 5 and round the half circle of radius 2
// about (10, 5) tangent to both, and the prism through it is cut from the block. Then the slot's
// three faces deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the slot runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of each of the
// block's six faces, read from the slotted block's own faces.
unit mm
use std
use std.solids
use std.solids (cut)

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

  m := point hint((10, 5))
  a distance(10mm, along: u) m
  a distance(5mm, along: v) m
  s0 := point hint((8, -1))
  s1 := point hint((12, -1))
  s2 := point hint((12, 5))
  s3 := point hint((8, 5))
  horizontal (mouth := line(s0, s1)) -> vertical (right := line(s1, s2)) -> tangent
  (bend := arc(center: m)) -> tangent vertical (left := line(s3, s0)) -> close
  radius(2) bend
  s1 distance(6) s2
  slot := solid(std.solids.Through(face(mouth, right, bend, left)))
  slot cut block
}

block := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 10mm))

part := solid({ p |
  p in block.near.side
  p in block.far.side
  p in block.south.side
  p in block.east.side
  p in block.north.side
  p in block.west.side
})
