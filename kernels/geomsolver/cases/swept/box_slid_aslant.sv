// The solid a box fills as it slides without turning along a line: the box from (0, 0, 0) to
// (10, 6, 4), its footprint in the top plane, carried along the vector (8, 5, 3), its whole length
// over the roll.
unit mm
use std
use std.solids
use std.motion (slides)

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 6))
  d := point hint((0, 6))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(6) c
}
box := solid(std.solids.Prism(footprint, from: 0mm, to: 4mm))

// the slide: along the line from the origin to (8, 5, 3), as far as that line is long
start := point
fix((0, 0, 0)) start
finish := point
fix((8, 5, 3)) finish
way := line(start, finish)
ram := frame
ram slides(sqrt(8^2 + 5^2 + 3^2) * 1mm) way
part := solid({ p | some t in (0deg, 360deg): p in box at(under: ram) t })
