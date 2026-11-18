// A rod threaded by a V wheel carried along a screw, as a thread is ground. The rod is the
// cylinder of radius 5 about z from z = 0 to 20. The wheel's axis runs through (14, 0, 0) along
// (0, cos h, -sin h), h = atan(2 pi 4.5 / 30): in the plane through that axis and x, the wheel is
// the triangle with its tip at (4, 0, 0) and its other corners 2.5 further out, 2.5 tan 30
// either side along the axis, turned about the axis. The screw is about z, right-handed, rising
// 30 a turn; the kernel's sweep of the wheel from a turn of -144 to 384 degrees is cut from the
// rod.
unit mm
use std
use std.solids (cut)
use std.place (coords, on_u)
use std.motion (screws)

h := atan(2 * pi * 4.5 / 30)     // the helix angle half a millimetre inside the rod

in std.top {
  private o := point
  fix((0, 0)) o
  private rod_rim := circle(center: o) hint(r: 5)
  radius(5mm) rod_rim
}
part := solid(std.solids.Prism(face(rod_rim), from: 0mm, to: 20mm))

// the wheel, drawn in the plane through its axis and x: the V's tip toward the rod
private across := axis
fix(dir == (1, 0, 0)) across
private along := axis
fix(dir == (0, cos(h), -sin(h))) along
bench := plane(u: across, v: along)
fix(origin == (0, 0, 0)) bench
in bench {
  private a0 := point
  private a1 := point
  a0 coords(du: 14mm, dv: -1mm) bench
  a1 coords(du: 14mm, dv: 1mm) bench
  private wheel_axis := line(a0, a1)
  private v0 := point
  private v1 := point
  private v2 := point
  v0 on_u(d: 4mm) bench
  v1 coords(du: 6.5mm, dv: -2.5mm * tan(30deg)) bench
  v2 coords(du: 6.5mm, dv: 2.5mm * tan(30deg)) bench
  private lower := line(v0, v1)
  private bore := line(v1, v2)
  private upper := line(v2, v0)
}
construction vwheel := solid(std.solids.Revolution(face(lower, bore, upper), about: wheel_axis))

grind := frame
grind screws(30mm) std.z
construction groove := solid({ p | some t in (-144deg, 384deg): p in vwheel at(under: grind) t })
groove cut part
