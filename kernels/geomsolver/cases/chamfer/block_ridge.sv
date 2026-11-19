// A block from the origin to (10, 4, 6): its 10 × 4 footprint drawn in the top plane (x, y), the
// prism over it to z = 6. The two edges of its top face (`near`) that run along x, where it meets
// the side swept from `south` (y = 0) and the side swept from `north` (y = 4), are chamfered 2 on
// each face, half the top's width: the two flats meet in a ridge along y = 2, z = 6, and the top
// face is gone.
unit mm
use std
use std.solids
use std.rounds (chamfer)

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 4))
  d := point hint((0, 4))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
  horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(4) c
  footprint := face(south, east, north, west)
}

part := solid(std.solids.Prism(footprint, from: 0mm, to: 6mm))
part.near chamfer(d: 2mm) part.south
part.near chamfer(d: 2mm) part.north
