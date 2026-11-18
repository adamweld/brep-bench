// A cube of side 10 from the origin, its square drawn in the top plane (x, y) and the prism over it
// from z = 0 to z = 10, united with the ball of radius 5 about its centre (5, 5, 5), which touches
// each of its six faces at one point.
unit mm
use std
use std.sets
use std.solids
use std.solids (union)

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 10))
  d := point hint((0, 10))
  square := horizontal line(a, b) -> vertical line(b, c) ->
            horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(10) c
}

o := point
fix((5, 5, 5)) o

ball := solid(std.sets.Ball(o, r: 5mm))
part := solid(std.solids.Prism(square, from: 0mm, to: 10mm))
ball union part
