// A plate, the box from (0, -5, 0) to (20, 5, 4), its footprint in the top plane; a drum lying on
// its side, the cylinder of radius 2 about the line y = 0, z = -5 from x = 6 to x = 14 (its end
// circle drawn in the side plane, carried along x), slid without turning 16 straight up, and
// everything it passes through cut from the plate.
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
  o := point
  fix((0, -5)) o
  rim := circle(center: o) hint(r: 2)
  radius(2) rim
}
drum := solid(std.solids.Prism(face(rim), from: 6mm, to: 14mm))

// the push, 16 straight up over the roll
ram := frame
ram slides(16mm) std.z
slot := solid({ p | some t in (0deg, 360deg): p in drum at(under: ram) t })
slot cut part
