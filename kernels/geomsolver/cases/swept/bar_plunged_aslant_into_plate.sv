// A plate, the box from (0, -5, 0) to (20, 5, 4), its footprint in the top plane; a bar of square
// section stood on a corner, |y| + |z + 6| <= 1 from x = 6 to x = 14 (its section drawn in the side
// plane through its corners, carried along x), slid without turning 8 along the direction
// (0.3, 0, sqrt(1 - 0.3^2)), stopping inside the plate, and everything it passes through cut from
// the plate.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (slides)

in std.top {
  a := point
  b := point hint((20, -5))
  c := point hint((20, 5))
  d := point hint((0, 5))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, -5)) a
  a distance(20) b
  b distance(10) c
}
part := solid(std.solids.Prism(outline, from: 0mm, to: 4mm))

in std.side {
  c0 := point
  c1 := point
  c2 := point
  c3 := point
  fix((0, -7)) c0
  fix((1, -6)) c1
  fix((0, -5)) c2
  fix((-1, -6)) c3
  square := line(c0, c1) -> line(c1, c2) -> line(c2, c3) -> line(c3, c0) -> close
}
bar := solid(std.solids.Prism(square, from: 6mm, to: 14mm))

// the push, 8 up a line leaning 0.3 toward x over the roll
aslant := axis hint(dir: (0.3, 0, 0.95))
fix(dir == (0.3, 0, sqrt(1 - 0.3^2))) aslant
ram := frame
ram slides(8mm) aslant
pocket := solid({ p | some t in (0deg, 360deg): p in bar at(under: ram) t })
pocket cut part
