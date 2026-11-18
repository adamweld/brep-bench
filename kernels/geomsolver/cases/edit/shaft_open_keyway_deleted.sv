// A shaft, the circle of radius 5 about the origin drawn in the top plane and its prism from
// z = 0 to z = 20, with an open keyway cut into it: the rectangle from x = −1 to x = 1, from
// y = −3 out past the shaft, drawn in the top plane, and its prism from z = 8 up past the
// shaft's top, cut away. Then all four of the keyway's faces deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit is
// said as one: deleting the keyway runs the faces round it on along their own surfaces until they
// meet, so what is left is the points on the material's side of the surface of every face of the
// keyed shaft but the keyway's (its two ends and its cylinder), read from its own faces.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim
}
shaft := solid(std.solids.Prism(face(rim), from: 0mm, to: 20mm))

in std.top {
  key0 := point hint((-1, -6))
  key1 := point hint((1, -6))
  key2 := point hint((1, -3))
  key3 := point hint((-1, -3))
  key := horizontal line(key0, key1) -> vertical line(key1, key2) ->
         horizontal line(key2, key3) -> vertical line(key3, key0) -> close
  fix((-1, -6)) key0
  fix((1, -3)) key2
}
keyway := solid(std.solids.Prism(key, from: 8mm, to: 21mm))
keyway cut shaft
part := solid({ p | p in shaft.near.side; p in shaft.far.side; p in shaft.rim.side })
