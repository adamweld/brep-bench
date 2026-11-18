// A block from (−10, −10, −10) to (10, 10, 0), its footprint drawn in the top plane and the prism
// under it, and the ball of radius 5 about the origin cut from it: a hemispherical pocket in its
// top face.
unit mm
use std
use std.solids
use std.solids (cut)
use std.sets

in std.top {
  a := point
  b := point hint((10, -10))
  c := point hint((10, 10))
  d := point hint((-10, 10))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-10, -10)) a
  a distance(20) b
  b distance(20) c
}

o := point
fix((0, 0, 0)) o
ball := solid(std.sets.Ball(o, r: 5mm))
part := solid(std.solids.Prism(footprint, from: -10mm, to: 0mm))
ball cut part
