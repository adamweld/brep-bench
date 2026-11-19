// A dumbbell-shaped plate shrunk by 1.5 until its neck is gone: its outline drawn in the top plane
// (x, y), two 10 × 10 squares joined by a neck 4 long and 2 wide, the prism over it to z = 6, and
// the points of it at least 1.5 from its outside, `std.rounds.Grown`'s body for a negative reach —
// two bodies, both kept.
unit mm
use std
use std.solids
use std.rounds

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 4))
  d := point hint((14, 4))
  e := point hint((14, 0))
  f := point hint((24, 0))
  g := point hint((24, 10))
  h := point hint((14, 10))
  i := point hint((14, 6))
  j := point hint((10, 6))
  k := point hint((10, 10))
  l := point hint((0, 10))
  outline := horizontal line(a, b) -> vertical line(b, c) -> horizontal line(c, d) ->
             vertical line(d, e) -> horizontal line(e, f) -> vertical line(f, g) ->
             horizontal line(g, h) -> vertical line(h, i) -> horizontal line(i, j) ->
             vertical line(j, k) -> horizontal line(k, l) -> vertical line(l, a) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(4) c
  c distance(4) d
  d distance(4) e
  e distance(10) f
  f distance(10) g
  g distance(10) h
  h distance(4) i
  i distance(4) j
  j distance(4) k
}

dumbbell := solid(std.solids.Prism(outline, from: 0mm, to: 6mm))
shrunk := std.rounds.Grown(dumbbell, d: -1.5mm)
part := solid(shrunk.body)
