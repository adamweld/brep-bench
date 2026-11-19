// A block, the box from (-6, -6, -8) to (18, 18, 0), its top in the top plane; a ball of radius 3,
// its centre 12 out along x on the top face, swung a quarter turn about the z axis, and everything
// it passes through cut from the block.
unit mm
use std
use std.sets
use std.solids
use std.solids (cut)
use std.motion (turns)

in std.top {
  a := point
  b := point hint((18, -6))
  c := point hint((18, 18))
  d := point hint((-6, 18))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-6, -6)) a
  a distance(24) b
  b distance(24) c
}
part := solid(std.solids.Prism(outline, from: -8mm, to: 0mm))

// the ball, about its centre on the top face
o := point
fix((12, 0, 0)) o
ball := solid(std.sets.Ball(o, r: 3mm))

// the quarter turn about the z axis, right-handed from +x toward +y
swing := frame
swing turns(1) std.z
groove := solid({ p | some t in (0deg, 90deg): p in ball at(under: swing) t })
groove cut part
