// A block, the box from (-3, -3, -3) to (3, 3, 3); a cube, the box from (-1, -1, -1) to (1, 1, 1),
// turned rigidly a whole turn about its long diagonal, the line through (-1, -1, -1) and (1, 1, 1),
// and everything it passes through cut from the block: a sealed void. Each box is its square
// footprint in the top plane, about the origin, carried along z.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (turns)
use std.shapes

in std.top {
  o := point
  fix((0, 0)) o
  outer := std.shapes.CenteredRectangle(o, w: 6mm, h: 6mm)
  inner := std.shapes.CenteredRectangle(o, w: 2mm, h: 2mm)
}
part := solid(std.solids.Prism(outer.loop, from: -3mm, to: 3mm))
cube := solid(std.solids.Prism(inner.loop, from: -1mm, to: 1mm))

low := point
fix((-1, -1, -1)) low
high := point
fix((1, 1, 1)) high
diagonal := line(low, high)
spin := frame
spin turns(1) diagonal
void := solid({ p | some t in (0deg, 360deg): p in cube at(under: spin) t })
void cut part
