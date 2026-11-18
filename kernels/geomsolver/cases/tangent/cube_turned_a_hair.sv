// A cube of side 10 from (-5, -5, 0) to (5, 5, 10), its square drawn about the origin of the top
// plane (x, y) and the prism over it from z = 0 to z = 10, united with a copy of itself turned
// 0.001° about the z axis: the copy's square drawn about the same centre, its sides turned 0.001°
// from the cube's, over the same two planes.
unit mm
use std
use std.shapes
use std.solids
use std.solids (union)

side := 10mm

in std.top {
  o := point
  fix((0, 0)) o
  cube := std.shapes.CenteredRectangle(o, w: side, h: side)

  e := point hint((-5, -5))
  f := point hint((5, -5))
  g := point hint((5, 5))
  h := point hint((-5, 5))
  turned := (ef := line(e, f)) -> (fg := line(f, g)) -> (gh := line(g, h)) -> line(h, e) -> close
  cube.ab angle(0.001deg) ef
  ef perpendicular fg
  fg perpendicular gh
  length(side) ef
  length(side) fg
  length(side) gh
  diagonal := line(e, g)
  o midpoint diagonal
}

copy := solid(std.solids.Prism(turned, from: 0mm, to: side))
part := solid(std.solids.Prism(cube.loop, from: 0mm, to: side))
copy union part
