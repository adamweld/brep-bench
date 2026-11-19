// A box from the origin to (20, 16, 12), its footprint drawn in the top plane (x, y) and the prism
// over it, every edge among its faces rounded to radius 2: twelve rounds along the edges and a
// ball's corner where three meet. Then all twenty rounds deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the rounds runs the faces round them on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of each of the
// box's six flat faces, read from the box's own faces.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  a := point
  b := point hint((20, 0))
  c := point hint((20, 16))
  d := point hint((0, 16))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
  horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  fix((0, 0)) a
  a distance(20) b
  b distance(16) c
}

box := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 12mm))
round(r: 2mm) box

part := solid({ p |
  p in box.near.side
  p in box.far.side
  p in box.south.side
  p in box.east.side
  p in box.north.side
  p in box.west.side
})
