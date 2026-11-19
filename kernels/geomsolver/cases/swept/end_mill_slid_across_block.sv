// A block, the box from (0, -5, 0) to (20, 5, 10), its footprint in the top plane; an end mill, the
// cylinder of radius 2 about the upright line through (-5, 0) from z = 7 to z = 15, slid without
// turning 30 along x, from clear before the block to clear past it, and everything it passes
// through cut from the block.
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

  o := point
  fix((-5, 0)) o
  rim := circle(center: o) hint(r: 2)
  radius(2) rim
}
part := solid(std.solids.Prism(outline, from: 0mm, to: 10mm))
mill := solid(std.solids.Prism(face(rim), from: 7mm, to: 15mm))

// the feed, 30 along x over the roll
feed := frame
feed slides(30mm) std.x
slot := solid({ p | some t in (0deg, 360deg): p in mill at(under: feed) t })
slot cut part
