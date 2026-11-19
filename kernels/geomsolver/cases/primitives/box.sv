// A box: its 10 × 20 footprint drawn in the top plane (x, y) from the origin, and the prism over
// it from z = 0 to z = 30 along the plane's normal.
unit mm
use std
use std.solids

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 20))
  d := point hint((0, 20))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(20) c
}

part := solid(std.solids.Prism(footprint, from: 0mm, to: 30mm))
