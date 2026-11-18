// A block, the box from (-10, -10, -5) to (10, 10, 0), its footprint in the top plane; an end mill,
// the cylinder of radius 3 about the upright line through (1.5, 0) from z = -2 to z = 4, so the z
// axis passes through its flat end; it is turned rigidly 120 degrees about the z axis,
// right-handed (from +x toward +y), and everything it passes through cut from the block.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (turns)

in std.top {
  a := point
  b := point hint((10, -10))
  c := point hint((10, 10))
  d := point hint((-10, 10))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-10, -10)) a
  a distance(20) b
  b distance(20) c

  o := point
  fix((1.5, 0)) o
  rim := circle(center: o) hint(r: 3)
  radius(3) rim
}
part := solid(std.solids.Prism(outline, from: -5mm, to: 0mm))
mill := solid(std.solids.Prism(face(rim), from: -2mm, to: 4mm))

swing := frame
swing turns(1) std.z
pocket := solid({ p | some t in (0deg, 120deg): p in mill at(under: swing) t })
pocket cut part
