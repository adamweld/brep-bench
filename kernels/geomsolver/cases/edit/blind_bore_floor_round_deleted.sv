// A block, its 20 × 20 footprint drawn in the top plane (x, y) about the origin and the prism over
// it to z = 10, with a blind bore of radius 4 about the z axis sunk from its top down to a floor
// at z = 4: the circle's prism from z = 4 up past the top, cut from the block. The concave edge
// where the floor (the cutter's `far`) meets the bore's wall (swept from `bore`) is rounded to
// radius 1, a fill. Then the round deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is bounded by the surfaces of every face of the bored block but the
// round's, read from its own faces: on the material's side of each of the block's six, and of
// the bore's wall or its floor.
unit mm
use std
use std.solids
use std.solids (cut)
use std.rounds (round)

in std.top {
  a := point
  b := point hint((10, -10))
  c := point hint((10, 10))
  d := point hint((-10, 10))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
  horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  fix((-10, -10)) a
  a distance(20) b
  b distance(20) c

  o := point
  fix((0, 0)) o
  bore := circle(center: o) hint(r: 4)
  radius(4) bore
  sink := solid(std.solids.Prism(face(bore), from: 4mm, to: 11mm))
  sink cut block
}

block := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 10mm))
block.sink.far round(r: 1mm) block.sink.bore

part := solid({ p |
  p in block.near.side
  p in block.far.side
  p in block.south.side
  p in block.east.side
  p in block.north.side
  p in block.west.side
  p in block.sink.bore.side or p in block.sink.far.side
})
