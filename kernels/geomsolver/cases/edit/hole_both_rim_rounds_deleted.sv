// A block, its 20 × 10 footprint drawn in the top plane (x, y) from the origin and the prism over
// it to z = 10, drilled right through along z by a hole of radius 2 about (10, 5): the circle's
// prism through the block, cut from it. Both of the hole's rims, where its wall meets the top
// (`near`) and the bottom (`far`), are rounded to radius 1. Then both rounds deleted together.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the rounds runs the faces round them on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the drilled block but the rounds' (the block's six and the hole's wall), read from its own
// faces.
unit mm
use std
use std.solids
use std.solids (cut)
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

  o := point hint((10, 5))
  a distance(10mm, along: u) o
  a distance(5mm, along: v) o
  hole := circle(center: o) hint(r: 2)
  radius(2) hole
  drill := solid(std.solids.Through(face(hole)))
  drill cut block
}

block := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 10mm))
block.near round(r: 1mm) block.drill
block.far round(r: 1mm) block.drill

part := solid({ p |
  p in block.near.side
  p in block.far.side
  p in block.south.side
  p in block.east.side
  p in block.north.side
  p in block.west.side
  p in block.drill.hole.side
})
