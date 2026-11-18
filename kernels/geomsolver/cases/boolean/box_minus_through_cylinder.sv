// A cube of side 10 from the origin, drilled along z: its footprint and the drill's circle, radius
// 3 about (5, 5), drawn in the top plane, the cube the prism over the footprint from z = 0 to 10
// and the drill the prism over the circle from z = −1 to 11, cut from it.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 10))
  d := point hint((0, 10))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(10) c

  o := point hint((5, 5))
  a distance(5mm, along: u) o
  a distance(5mm, along: v) o
  rim := circle(center: o) hint(r: 3)
  radius(3) rim
}

drill := solid(std.solids.Prism(face(rim), from: -1mm, to: 11mm))
part := solid(std.solids.Prism(footprint, from: 0mm, to: 10mm))
drill cut part
