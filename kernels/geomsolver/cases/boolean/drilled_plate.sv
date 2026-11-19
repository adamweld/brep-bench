// A plate, 100 × 20 in the top plane (x, y) and 10 thick, drilled nine times along z: each hole a
// circle of radius 2 at y = 10, x = 10, 20, … 90, and its own cutter through the plate.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  a := point
  b := point hint((100, 0))
  c := point hint((100, 20))
  d := point hint((0, 20))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(100) b
  b distance(20) c

  repeat 9 as i {
    o := point hint((10 * (i + 1), 10))
    a distance(10mm * (i + 1), along: u) o
    a distance(10mm, along: v) o
    hole := circle(center: o) hint(r: 2)
    radius(2) hole
    drill := solid(std.solids.Through(face(hole)))
    drill cut part
  }
}

part := solid(std.solids.Prism(outline, from: 0mm, to: 10mm))
