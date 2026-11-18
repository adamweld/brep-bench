// A block, the box from (0, -5, 0) to (20, 5, 10), with a V groove cut across its top by a sharp
// V wheel slid along it. The wheel's axis is the line through (-8, 0, 14) along y: in the plane
// through that axis and z, the wheel is the triangle with its tip 6 below the axis (2 into the
// block) and its other corners 3 above the tip, 3 tan 30 either side along the axis, turned about
// the axis. It slides 36 along x, and the kernel's sweep of it is cut from the block.
unit mm
use std
use std.solids (cut)
use std.place (coords, on_u)
use std.shapes
use std.motion (slides)

in std.top {
  private mid := point
  fix((10, 0)) mid
  private outline := std.shapes.CenteredRectangle(mid, w: 20mm, h: 10mm)
}
part := solid(std.solids.Prism(outline.loop, from: 0mm, to: 10mm))

// the wheel, drawn in the plane through its axis, out from the axis downward
private down := axis
fix(dir == (0, 0, -1)) down
private along := axis
fix(dir == (0, 1, 0)) along
bench := plane(u: down, v: along)
fix(origin == (-8, 0, 14)) bench
in bench {
  private a0 := point
  private a1 := point
  a0 coords(du: 0mm, dv: -1mm) bench
  a1 coords(du: 0mm, dv: 1mm) bench
  private wheel_axis := line(a0, a1)
  private v0 := point
  private v1 := point
  private v2 := point
  v0 on_u(d: 6mm) bench
  v1 coords(du: 3mm, dv: -3mm * tan(30deg)) bench
  v2 coords(du: 3mm, dv: 3mm * tan(30deg)) bench
  private lower := line(v0, v1)
  private bore := line(v1, v2)
  private upper := line(v2, v0)
}
construction vwheel := solid(std.solids.Revolution(face(lower, bore, upper), about: wheel_axis))

feed := frame
feed slides(36mm) std.x
construction groove := solid({ p | some t in (0deg, 360deg): p in vwheel at(under: feed) t })
groove cut part
