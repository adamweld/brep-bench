// A cube tumbled through space: the box from (2.4, -0.6, 1.4) to (3.6, 0.6, 2.6), its footprint
// in the top plane and the prism over it, turned t/4 about A, the upright line through (2, 0, 0),
// and then -t about B, the line through (0, 0.5, 0.5) along x. The part is the kernel's sweep of
// the cube as t runs from -75 to 75 degrees, read as the cube turning about A seen from a frame
// turning +t about B.
unit mm
use std
use std.solids
use std.shapes
use std.motion (turns)

in std.top {
  private mid := point
  fix((3, 0)) mid
  private outline := std.shapes.CenteredRectangle(mid, w: 1.2mm, h: 1.2mm)
}
construction cube := solid(std.solids.Prism(outline.loop, from: 1.4mm, to: 2.6mm))

// A: the upright line through (2, 0, 0)
private a_foot := point
fix((2, 0, 0)) a_foot
private a := axis
fix(dir == (0, 0, 1)) a
a_foot coincident a
// B: the line through (0, 0.5, 0.5) along x
private q := point
fix((0, 0.5, 0.5)) q
private b := axis
fix(dir == (1, 0, 0)) b
q coincident b

private spin := frame
spin turns(0.25) a
private observer := frame
observer turns(1) b
part := solid({ p | some t in (-75deg, 75deg): p in cube at(under: spin, seen_from: observer) t })
