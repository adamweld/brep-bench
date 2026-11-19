// A plate, the box from (-15, -15, 0) to (15, 15, 4), with a hole punched by a tilted box pushed
// aslant wholly through it. The box is 4 x 3 x 2 about (-2, -1, -8), its edges along
// e1 = (2, 1, 2)/3, e2 = (-2, 2, 1)/3 and e3 = (1, 2, -2)/3: its 4 x 3 face drawn in the plane
// through its centre spanned by e1 and e2, and the prism over it 1 either side. It slides without
// turning along (4, 2, 18), and the kernel's sweep of it under that slide is cut from the plate.
unit mm
use std
use std.solids (cut)
use std.shapes
use std.motion (slides)

in std.top {
  private mid := point
  fix((0, 0)) mid
  private outline := std.shapes.CenteredRectangle(mid, w: 30mm, h: 30mm)
}
part := solid(std.solids.Prism(outline.loop, from: 0mm, to: 4mm))

// the box, about its centre in the plane of its first two edges
private e1 := axis
fix(dir == (2, 1, 2)) e1
private e2 := axis
fix(dir == (-2, 2, 1)) e2
bench := plane(u: e1, v: e2)
fix(origin == (-2, -1, -8)) bench
in bench {
  private c := point
  fix((0, 0)) c
  private section := std.shapes.CenteredRectangle(c, w: 4mm, h: 3mm)
}
construction box := solid(std.solids.Prism(section.loop, from: -1mm, to: 1mm))

// the push: along (4, 2, 18) from the box's centre, its whole length over the roll's one turn
private push := axis
fix(dir == (4, 2, 18)) push
c coincident push
ram := frame
ram slides(sqrt(4^2 + 2^2 + 18^2) * 1mm) push
construction hole := solid({ p | some t in (0deg, 360deg): p in box at(under: ram) t })
hole cut part
