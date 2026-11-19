// A rod grooved by a wheel of two beads slid along it. The rod is the cylinder of radius 5 about
// z from z = 0 to 20. The beads are two tori on one axis, along y through (11.5, 0, -9): the
// first centred there, its tube (radius 2) 6 from the axis; the second centred 1.5 further along
// the axis, its tube (radius 1.5) also 6 from it. Each is its tube's circle drawn in the plane
// through the axis and x and turned about the axis; the wheel is their union. It slides 40 up z,
// and the kernel's sweep of the wheel is cut from the rod.
unit mm
use std
use std.solids (cut, union)
use std.place (coords, on_u)
use std.motion (slides)

in std.top {
  private o := point
  fix((0, 0)) o
  private rod_rim := circle(center: o) hint(r: 5)
  radius(5mm) rod_rim
}
part := solid(std.solids.Prism(face(rod_rim), from: 0mm, to: 20mm))

// the beads, drawn in the level plane through their axis, below the rod
private across := axis
fix(dir == (1, 0, 0)) across
private along := axis
fix(dir == (0, 1, 0)) along
bench := plane(u: across, v: along)
fix(origin == (0, 0, -9)) bench
in bench {
  private a0 := point
  private a1 := point
  a0 coords(du: 11.5mm, dv: -1mm) bench
  a1 coords(du: 11.5mm, dv: 1mm) bench
  private wheel_axis := line(a0, a1)
  private tc := point
  tc on_u(d: 5.5mm) bench
  private tube := circle(center: tc) hint(r: 2)
  radius(2mm) tube
  private tc2 := point
  tc2 coords(du: 5.5mm, dv: 1.5mm) bench
  private tube2 := circle(center: tc2) hint(r: 1.5)
  radius(1.5mm) tube2
}
construction bead := solid(std.solids.Revolution(face(tube), about: wheel_axis))
construction bead2 := solid(std.solids.Revolution(face(tube2), about: wheel_axis))
construction wheel := solid(bead)
bead2 union wheel

feed := frame
feed slides(40mm) std.z
construction groove := solid({ p | some t in (0deg, 360deg): p in wheel at(under: feed) t })
groove cut part
