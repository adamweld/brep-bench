// A cube of side 10 from the origin, its footprint drawn in the top plane (x, y) and the prism
// over it to z = 10; every edge among its faces chamfered 2 on each face. Each edge's flat is its
// own, cut from the cube as the cube stands, so the three at a corner run on until they meet in
// a point: the corner mitred.
unit mm
use std
use std.solids
use std.rounds (chamfer)

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
chamfer(d: 2mm) part
