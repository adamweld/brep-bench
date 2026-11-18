// A block, its 20 × 10 footprint drawn in the top plane (x, y) from the origin and the prism over
// it to z = 10, with a half-round notch in its side y = 0: the circle of radius 3 about (10, 0),
// on the side's own line, and the prism through it cut from the block. Then the notch's wall
// deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the wall runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of each of the
// block's six faces (the side y = 0 in two pieces), read from the notched block's own faces.
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

  m := point hint((10, 0))
  m coincident south
  a distance(10mm, along: u) m
  wall := circle(center: m) hint(r: 3)
  radius(3) wall
  notch := solid(std.solids.Through(face(wall)))
  notch cut block
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
