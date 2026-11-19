// A small cube of side 0.0015 from the origin, its square drawn in the top plane (x, y) and the
// prism over it from z = 0 to z = 0.0015, and a copy of itself raised 0.000015 along z: the prism
// over the same square from z = 0.000015 to z = 0.001515. Their common part.
unit mm
use std
use std.solids
use std.solids (common)

side := 0.0015mm

in std.top {
  a := point
  b := point hint((0.0015, 0))
  c := point hint((0.0015, 0.0015))
  d := point hint((0, 0.0015))
  square := horizontal line(a, b) -> vertical line(b, c) ->
            horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(side) b
  b distance(side) c
}

copy := solid(std.solids.Prism(square, from: 0.000015mm, to: side + 0.000015mm))
part := solid(std.solids.Prism(square, from: 0mm, to: side))
copy common part
