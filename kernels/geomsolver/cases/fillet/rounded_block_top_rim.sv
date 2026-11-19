// A rounded block: the rectangle from (−10, −6) to (10, 6) drawn in the top plane by its corners,
// each corner a round of radius 3 tangent to the sides it joins, and the prism over it from z = 0
// to z = 5. The rim of its top (`near`) — every edge where it meets another face, four straight
// runs and four arcs, tangent all the way round — rounded to radius 1.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  a := point hint((-10, -6))
  b := point hint((10, -6))
  c := point hint((10, 6))
  d := point hint((-10, 6))
  outline := horizontal (south := line(a, b)) -> round(r: 3mm) vertical (east := line(b, c)) ->
             round(r: 3mm) horizontal (north := line(c, d)) -> round(r: 3mm)
             vertical (west := line(d, a)) -> round(r: 3mm) close
  fix((-10, -6)) a
  a distance(20mm) b
  b distance(12mm) c
}

part := solid(std.solids.Prism(outline, from: 0mm, to: 5mm))
part.near round(r: 1mm) part
