// The solid a rounded block fills as it slides aslant. The block is the box from (0, 0, 0) to
// (10, 6, 4), its footprint in the top plane and the prism over it, with every edge rounded to
// radius 1 (three rounds meeting in a ball at each corner). The part is the kernel's sweep of the
// block slid without turning along (8, 5, 3).
unit mm
use std
use std.solids
use std.rounds (round)
use std.motion (slides)

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 6))
  d := point hint((0, 6))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(10mm) b
  b distance(6mm) c
}
construction block := solid(std.solids.Prism(footprint, from: 0mm, to: 4mm))
round(r: 1mm) block

// the slide: along (8, 5, 3) from the corner, its whole length over the roll's one turn
private path := axis
fix(dir == (8, 5, 3)) path
a coincident path
carriage := frame
carriage slides(sqrt(8^2 + 5^2 + 3^2) * 1mm) path
part := solid({ p | some t in (0deg, 360deg): p in block at(under: carriage) t })
