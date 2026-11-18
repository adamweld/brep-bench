// A rod threaded by a square bar carried along a screw, as a lathe cuts a square thread. The rod is
// the cylinder of radius 5 about the z axis from z = 0 to 20. The bar is the box 4 <= x <= 12,
// -8 <= y <= 8, -1 <= z <= 1: its section drawn in the front plane (x, z), dimensioned from the
// rod's axis, and the prism over it 8 either side. The screw is about z, right-handed, rising 6 a
// turn; the kernel's sweep of the bar from a turn of -90 to 1350 degrees is cut from the rod.
unit mm
use std
use std.solids (cut)
use std.place (coords, above)
use std.motion (screws)

in std.top {
  private o := point
  fix((0, 0)) o
  private rim := circle(center: o) hint(r: 5)
  radius(5mm) rim
}
part := solid(std.solids.Prism(face(rim), from: 0mm, to: 20mm))

in std.front {
  // the rod's axis, up from its foot at the origin
  construction centerline ax := line(std.origin)
  ax.p2 above(d: 20mm) std.origin
  // the bar's section, from the axis: 4 to 12 out along x, 1 either side of z = 0
  private b0 := point
  private b1 := point
  private b2 := point
  private b3 := point
  b0 coords(du: 4mm, dv: -1mm) std.front
  b1 coords(du: 12mm, dv: -1mm) std.front
  b2 coords(du: 12mm, dv: 1mm) std.front
  b3 coords(du: 4mm, dv: 1mm) std.front
  private lower := line(b0, b1)
  private end := line(b1, b2)
  private upper := line(b2, b3)
  private nose := line(b3, b0)
}
construction bar := solid(std.solids.Prism(face(lower, end, upper, nose), from: -8mm, to: 8mm))

// the screw about the rod's axis, carrying the bar from below the rod to above it
feed := frame
feed screws(6mm) ax
construction thread := solid({ p | some t in (-90deg, 1350deg): p in bar at(under: feed) t })
thread cut part
