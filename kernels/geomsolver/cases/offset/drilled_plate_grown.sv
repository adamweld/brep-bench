// The plate from (−10, −10, 0) to (10, 10, 4), drilled through along the z axis by a hole of
// radius 3, grown by 1: the points within 1 of the drilled plate, `std.rounds.Grown`'s body.
unit mm
use std
use std.solids
use std.solids (cut)
use std.rounds

in std.top {
  a := point
  b := point hint((10, -10))
  c := point hint((10, 10))
  d := point hint((-10, 10))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-10, -10)) a
  a distance(20) b
  b distance(20) c

  o := point
  fix((0, 0)) o
  hole := circle(center: o) hint(r: 3)
  radius(3) hole
  drill := solid(std.solids.Through(face(hole)))
  drill cut plate
}

plate := solid(std.solids.Prism(outline, from: 0mm, to: 4mm))
grown := std.rounds.Grown(plate, d: 1mm)
part := solid(grown.body)
