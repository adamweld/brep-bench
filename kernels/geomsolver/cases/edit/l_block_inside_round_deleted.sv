// An L-shaped block: the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15)
// drawn in the top plane (x, y), the prism over it to z = 8, its inside corner, the concave edge
// where the tread y = 5 (swept from `tread`) meets the riser x = 5 (from `riser`), rounded to
// radius 2, a fill. Then the round deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is bounded by the surfaces of every face of the block but the
// round's, read from the block's own faces. The block is not on one side of all of them, so it is
// said as its two arms: the foot, on the material's side of every face but the riser and the
// far end, and the upright, of every face but the tread and the far side.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  a := point
  b := point hint((20, 0))
  c := point hint((20, 5))
  d := point hint((5, 5))
  e := point hint((5, 15))
  f := point hint((0, 15))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
  horizontal (tread := line(c, d)) -> vertical (riser := line(d, e)) ->
  horizontal (north := line(e, f)) -> vertical (west := line(f, a)) -> close
  fix((0, 0)) a
  a distance(20) b
  b distance(5) c
  a distance(15) f
  f distance(5) e
}

block := solid(std.solids.Prism(face(south, east, tread, riser, north, west), from: 0mm, to: 8mm))
block.tread round(r: 2mm) block.riser

foot := { q |
  q in block.near.side
  q in block.far.side
  q in block.south.side
  q in block.east.side
  q in block.tread.side
  q in block.west.side
}
upright := { q |
  q in block.near.side
  q in block.far.side
  q in block.south.side
  q in block.riser.side
  q in block.north.side
  q in block.west.side
}
part := solid({ p | p in foot or p in upright })
