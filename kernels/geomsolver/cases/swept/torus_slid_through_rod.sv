// A rod grooved by a wheel slid along it. The rod is the cylinder of radius 5 about z from z = 0
// to 20. The wheel is a torus centred at (11.5, 0, -9), its axis along y, its tube (radius 2) 6
// from that axis: the tube's circle drawn in the plane through the axis and x, 5.5 out from z, and
// turned about the axis. It slides 38 up z, and the kernel's sweep of it is cut from the rod.
unit mm
use std
use std.solids (cut)
use std.place (coords, on_u)
use std.motion (slides)

in std.top {
  private o := point
  fix((0, 0)) o
  private rod_rim := circle(center: o) hint(r: 5)
  radius(5mm) rod_rim
}
part := solid(std.solids.Prism(face(rod_rim), from: 0mm, to: 20mm))

// the wheel, drawn in the level plane through its axis, below the rod
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
}
construction wheel := solid(std.solids.Revolution(face(tube), about: wheel_axis))

// the slide up z, 38 over the roll's one turn, from wholly below the rod to wholly above it
feed := frame
feed slides(38mm) std.z
construction groove := solid({ p | some t in (0deg, 360deg): p in wheel at(under: feed) t })
groove cut part
