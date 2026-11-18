// A rod grooved by a wheel screwed along it, as a thread is ground. The rod is the cylinder of
// radius 5 about z from z = 0 to 20. The wheel is a torus centred at (11.5, 0, 0), its tube
// (radius 2) 6 from its axis, which leans from y down toward -z by the helix angle
// h = atan(2 pi 5.5 / 30): the tube's circle drawn in the plane through that axis and x, 5.5 out
// from z, and turned about the axis. The screw is about z, right-handed, rising 30 a turn; the
// kernel's sweep of the wheel from a turn of -108 to 348 degrees is cut from the rod.
unit mm
use std
use std.solids (cut)
use std.place (coords, on_u)
use std.motion (screws)

h := atan(2 * pi * 5.5 / 30)     // the helix angle where the tube's centre runs

in std.top {
  private o := point
  fix((0, 0)) o
  private rod_rim := circle(center: o) hint(r: 5)
  radius(5mm) rod_rim
}
part := solid(std.solids.Prism(face(rod_rim), from: 0mm, to: 20mm))

// the wheel, drawn in the plane through its axis and the x axis
private across := axis
fix(dir == (1, 0, 0)) across
private along := axis
fix(dir == (0, cos(h), -sin(h))) along
bench := plane(u: across, v: along)
fix(origin == (0, 0, 0)) bench
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

grind := frame
grind screws(30mm) std.z
construction groove := solid({ p | some t in (-108deg, 348deg): p in wheel at(under: grind) t })
groove cut part
