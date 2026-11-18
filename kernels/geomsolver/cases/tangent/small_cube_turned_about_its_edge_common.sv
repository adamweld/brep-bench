// A small cube of side 0.001 from the origin, its square drawn in the top plane (x, y) and the prism
// over it from z = 0 to z = 0.001, and a copy of itself turned 1.2° about the z axis, the line of
// one of its own edges: the copy's square drawn from the same corner, its first side turned 1.2°
// from the cube's, over the same two planes. Their common part.
unit mm
use std
use std.solids
use std.solids (common)

side := 0.001mm

in std.top {
  a := point
  b := point hint((0.001, 0))
  c := point hint((0.001, 0.001))
  d := point hint((0, 0.001))
  square := horizontal (ab := line(a, b)) -> vertical line(b, c) ->
            horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(side) b
  b distance(side) c

  e := point hint((0.001, 0.00002))
  f := point hint((0.00098, 0.00102))
  g := point hint((-0.00002, 0.001))
  turned := (ef := line(a, e)) -> (fg := line(e, f)) -> (gh := line(f, g)) -> (ha := line(g, a)) -> close
  ab angle(1.2deg) ef
  ef perpendicular fg
  fg perpendicular gh
  length(side) ef
  length(side) fg
  length(side) gh
}

copy := solid(std.solids.Prism(turned, from: 0mm, to: side))
part := solid(std.solids.Prism(square, from: 0mm, to: side))
copy common part
