// A cube of side 10 from the origin, its square drawn in the top plane (x, y) and the prism over it
// from z = 0 to z = 10, less a saw cut a millionth wide: a block between x = 5 and x = 5.000001,
// its rectangle drawn in the same plane reaching past the cube's front and back, from y = -1 to
// y = 11, and the prism over it from z = 5 up past the top, to z = 11.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 10))
  d := point hint((0, 10))
  square := horizontal line(a, b) -> vertical line(b, c) ->
            horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(10) c

  e := point hint((5, -1))
  f := point hint((5.000001, -1))
  g := point hint((5.000001, 11))
  h := point hint((5, 11))
  slot := horizontal line(e, f) -> vertical line(f, g) ->
          horizontal line(g, h) -> vertical line(h, e) -> close
  a distance(5, along: u) e
  a distance(-1, along: v) e
  e distance(0.000001mm, along: u) f
  f distance(12, along: v) g
}

saw := solid(std.solids.Prism(slot, from: 5mm, to: 11mm))
part := solid(std.solids.Prism(square, from: 0mm, to: 10mm))
saw cut part
