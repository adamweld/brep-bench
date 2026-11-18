// An L-shaped plate grown by 1: its outline drawn in the top plane (x, y) through (0, 0), (20, 0),
// (20, 5), (5, 5), (5, 15) and (0, 15), the prism over it to z = 8, and the points within 1 of it,
// `std.rounds.Grown`'s body: the convex edges rounded, the inside corner's concave edge left sharp.
unit mm
use std
use std.solids
use std.rounds

in std.top {
  a := point
  b := point hint((20, 0))
  c := point hint((20, 5))
  d := point hint((5, 5))
  e := point hint((5, 15))
  f := point hint((0, 15))
  outline := horizontal line(a, b) -> vertical line(b, c) -> horizontal line(c, d) ->
             vertical line(d, e) -> horizontal line(e, f) -> vertical line(f, a) -> close
  fix((0, 0)) a
  a distance(20) b
  b distance(5) c
  c distance(15) d
  f distance(15) a
}

plate := solid(std.solids.Prism(outline, from: 0mm, to: 8mm))
grown := std.rounds.Grown(plate, d: 1mm)
part := solid(grown.body)
