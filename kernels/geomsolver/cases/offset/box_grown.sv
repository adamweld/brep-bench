// The box from the origin to (10, 8, 6), grown by 1: its footprint drawn in the top plane (x, y),
// the prism over it to z = 6, and the points within 1 of it, `std.rounds.Grown`'s body.
unit mm
use std
use std.solids
use std.rounds

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 8))
  d := point hint((0, 8))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(8) c
}

box := solid(std.solids.Prism(footprint, from: 0mm, to: 6mm))
grown := std.rounds.Grown(box, d: 1mm)
part := solid(grown.body)
