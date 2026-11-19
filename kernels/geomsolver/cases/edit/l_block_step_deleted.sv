// An L-shaped block: the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15)
// drawn in the top plane (x, y), the prism over it to z = 8, its step the tread y = 5 (swept from
// `tread`) and the riser x = 5 (from `riser`). Then the tread and the riser deleted together.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the step runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the block but the tread and the riser, read from the block's own faces. The far side x = 20
// and the far end y = 15 run on and meet.
unit mm
use std
use std.solids

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

part := solid({ p |
  p in block.near.side
  p in block.far.side
  p in block.south.side
  p in block.east.side
  p in block.north.side
  p in block.west.side
})
