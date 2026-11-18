// A cube of side 10 from the origin, its footprint in the top plane (x, y) and the prism over it
// to z = 10; the edge where its x = 10 face (swept from `east`) meets its y = 10 face (from
// `north`) rounded to radius 2.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 10))
  d := point hint((0, 10))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
  horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(10) c
  footprint := face(south, east, north, west)
}

part := solid(std.solids.Prism(footprint, from: 0mm, to: 10mm))
part.east round(r: 2mm) part.north
