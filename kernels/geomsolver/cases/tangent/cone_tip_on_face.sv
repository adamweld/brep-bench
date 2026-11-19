// A block from (-10, -10, 0) to (10, 10, 10), its square drawn in the top plane (x, y) and the prism
// over it, united with a cone on the z axis standing on its tip at the origin, the middle of the
// block's bottom face, its flat base the disk of radius 5 at z = 15: the cone the half-section, a
// right triangle drawn in the front plane (x, z) from the axis, turned a whole turn about z.
unit mm
use std
use std.solids
use std.solids (union)

in std.top {
  a := point
  b := point hint((10, -10))
  c := point hint((10, 10))
  d := point hint((-10, 10))
  square := horizontal line(a, b) -> vertical line(b, c) ->
            horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-10, -10)) a
  a distance(20) b
  b distance(20) c
}

in std.front {
  tip := point
  axis_top := point hint((0, 15))
  rim := point hint((5, 15))
  fix((0, 0)) tip
  tip distance(15) axis_top
  axis_top distance(5) rim
  half := line(tip, rim) -> horizontal line(rim, axis_top) -> vertical line(axis_top, tip) -> close
}

cone := solid(std.solids.Revolution(half, about: std.z))
part := solid(std.solids.Prism(square, from: 0mm, to: 10mm))
cone union part
