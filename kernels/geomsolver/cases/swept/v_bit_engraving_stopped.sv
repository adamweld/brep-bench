// A block, the box from (0, -10, -10) to (30, 10, 0), with a line engraved into its top by a V
// bit stopping inside it. The bit is a cone about the vertical through (5, 0): its point at
// (5, 0, -2), opening at 45 degrees to a flat top of radius 5 at z = 3, drawn as its half section
// in the front plane and turned about its axis. It slides 15 along x, and the kernel's sweep of
// it is cut from the block.
unit mm
use std
use std.solids (cut)
use std.place (coords, above, horizontal)
use std.shapes
use std.motion (slides)

in std.top {
  private mid := point
  fix((15, 0)) mid
  private outline := std.shapes.CenteredRectangle(mid, w: 30mm, h: 20mm)
}
part := solid(std.solids.Prism(outline.loop, from: -10mm, to: 0mm))

in std.front {
  private tip := point
  private lip := point hint((10, 3))
  private back := point hint((5, 3))
  tip coords(du: 5mm, dv: -2mm) std.front
  back above(d: 5mm) tip
  construction centerline spindle := line(tip, back)
  (cone := line(tip, lip)) angle(45deg) spindle
  horizontal (top := line(lip, back))
}
construction bit := solid(std.solids.Revolution(face(cone, top, spindle), about: spindle))

feed := frame
feed slides(15mm) std.x
construction groove := solid({ p | some t in (0deg, 360deg): p in bit at(under: feed) t })
groove cut part
