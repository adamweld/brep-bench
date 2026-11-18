// A block, its 20 × 10 footprint drawn in the top plane (x, y) from the origin and the prism over
// it to z = 10, drilled right through along z by a hole of radius 2 about (10, 5), and countersunk
// at the top: the rim of radius 4 drawn on a plane lying on the top where the hole's axis meets
// it, and the 90° countersink under it cut too, so the cone runs in to the hole's radius at
// z = 8. Then the countersink's cone deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the cone runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the countersunk block but the cone (the block's six and the hole's wall), read from its own
// faces.
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

  o := point hint((10, 5))
  a distance(10mm, along: u) o
  a distance(5mm, along: v) o
  hole := circle(center: o) hint(r: 2)
  radius(2) hole
  drill := solid(std.solids.Through(face(hole)))
  drill cut block
}

block := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 10mm))

// the hole's axis, and the plane lying on the top where it meets it
o_up := point hint((10, 5, 1))
o_up distance(1mm, along: n) std.top
axis_line := line(o, o_up)
axis_line perpendicular std.top
deck := plane(u: hint(dir: (1, 0, 0)), v: hint(dir: (0, 1, 0))) hint(origin: (10, 5, 10))
deck.origin in boundary(block.near.side)
deck tangent(at: deck.origin) boundary(block.near.side)
deck.origin coincident axis_line
deck.u parallel std.x
in deck {
  rim := circle(center: deck.origin) hint(r: 4)
  radius(4mm) rim
}
sink := solid(std.solids.Countersink(face(rim), angle: 90deg))
sink cut block

part := solid({ p |
  p in block.near.side
  p in block.far.side
  p in block.south.side
  p in block.east.side
  p in block.north.side
  p in block.west.side
  p in block.drill.hole.side
})
