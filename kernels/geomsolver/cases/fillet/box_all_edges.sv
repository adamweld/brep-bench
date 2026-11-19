// A cube of side 10, every edge among its faces rounded to radius 1: three rounds meet in a ball's
// corner at each vertex.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 10))
  d := point hint((0, 10))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(10) c
}

part := solid(std.solids.Prism(footprint, from: 0mm, to: 10mm))
round(r: 1mm) part
