// A rod grooved all round by a leaning V wheel turned once round it, as a rod turns in a lathe
// against a form tool. The rod is the cylinder of radius 5 about z from z = 0 to 20. The wheel's
// axis runs through (14, 0, 10) along (0, sin 20, cos 20): in the plane through that axis and x,
// the wheel is the triangle with its tip at (4, 0, 10) and its other corners 2.5 further out,
// 2.5 tan 30 either side along the axis, turned about the axis. The kernel's sweep of the wheel
// turned a whole turn about z, from where it is drawn, is cut from the rod.
unit mm
use std
use std.solids (cut)
use std.place (coords, on_u)
use std.motion (turns)

lean := 20deg        // how far the wheel's axis leans from upright, across the turn

in std.top {
  private o := point
  fix((0, 0)) o
  private rod_rim := circle(center: o) hint(r: 5)
  radius(5mm) rod_rim
}
part := solid(std.solids.Prism(face(rod_rim), from: 0mm, to: 20mm))

// the wheel, drawn in the plane through its axis and x at the rod's middle height
private across := axis
fix(dir == (1, 0, 0)) across
private along := axis
fix(dir == (0, sin(lean), cos(lean))) along
bench := plane(u: across, v: along)
fix(origin == (0, 0, 10)) bench
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

lathe := frame
lathe turns(1) std.z
construction groove := solid({ p | some t in (0deg, 360deg): p in vwheel at(under: lathe) t })
groove cut part
