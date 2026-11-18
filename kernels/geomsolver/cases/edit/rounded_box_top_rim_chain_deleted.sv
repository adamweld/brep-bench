// A block, its 20 × 12 footprint drawn in the top plane (x, y) from the origin and the prism over
// it to z = 8, its four upright edges rounded to radius 3, each round named for its corner. Then,
// on a solid of the block as those rounds leave it, the whole rim of its top (`near`) rounded to
// radius 1 against every face of it: one tangent chain of eight faces, along the four flat sides
// and round the four corners' cylinders. Then the eight faces of the chain deleted together.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the chain runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the top's, the bottom's and
// the four flat sides' surfaces, and of each corner's cylinder where that cylinder is nearer than
// the flat sides it runs between: a corner's cylinder bounds the block only round its corner.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  a := point
  b := point hint((20, 0))
  c := point hint((20, 12))
  d := point hint((0, 12))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
  horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  fix((0, 0)) a
  a distance(20) b
  b distance(12) c
}

box := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 8mm))
se := box.south round(r: 3mm) box.east
ne := box.east round(r: 3mm) box.north
nw := box.north round(r: 3mm) box.west
sw := box.west round(r: 3mm) box.south
rimmed := solid(box)
rimmed.box.near round(r: 1mm) rimmed.box

// the points at least as near the face `f` as the face `c`
component Nearer(c: face, f: face) := { q | distance(q, c) >= distance(q, f) }
// the points within a corner's cylinder `c`, or nearer either flat side `a` and `b` it runs between
component Corner(c: face, a: face, b: face) := { q |
  q in c.side or q in Nearer(c, a) or q in Nearer(c, b)
}

part := solid({ p |
  p in rimmed.box.near.side
  p in rimmed.box.far.side
  p in rimmed.box.south.side
  p in rimmed.box.east.side
  p in rimmed.box.north.side
  p in rimmed.box.west.side
  p in Corner(rimmed.box.se, rimmed.box.south, rimmed.box.east)
  p in Corner(rimmed.box.ne, rimmed.box.east, rimmed.box.north)
  p in Corner(rimmed.box.nw, rimmed.box.north, rimmed.box.west)
  p in Corner(rimmed.box.sw, rimmed.box.west, rimmed.box.south)
})
