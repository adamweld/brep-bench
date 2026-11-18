// A rod threaded by a square bar at a lead that grows as it climbs. The rod is the cylinder of
// radius 5 about z from z = 0 to 20. The bar is the box 4 <= x <= 12, -8 <= y <= 8, -1 <= z <= 1
// as placed: its section in a plane through the rod's axis and the prism over it 8 either side.
// No screw carries it, so the motion is a frame the drawing moves: with one parameter t, the
// frame `carrier` stands turned t about z and risen 6 s + 0.3 s^2 (s = t / 360 degrees), and the
// kernel's sweep of the bar under it as t runs from -90 to 1260 degrees is cut from the rod. Such
// a frame carries what is drawn from where it stands as drawn, at the parameter's first value, so
// the bar is drawn where it starts.
unit mm
use std
use std.solids (cut)
use std.place (coords)

param t: Angle over (-90deg, 1260deg)

in std.top {
  private o := point
  fix((0, 0)) o
  private rim := circle(center: o) hint(r: 5)
  radius(5mm) rim
}
part := solid(std.solids.Prism(face(rim), from: 0mm, to: 20mm))

// the bar drawn where it starts, t = -90deg: turned -90deg about z, its middle risen
// 6 s + 0.3 s^2 at s = -1/4
private start_u := axis
fix(dir == (0, -1, 0)) start_u
private start_v := axis
fix(dir == (0, 0, 1)) start_v
holder := plane(u: start_u, v: start_v)
fix(origin == (0, 0, -6 / 4 + 0.3 / 16)) holder
in holder {
  private b0 := point
  private b1 := point
  private b2 := point
  private b3 := point
  b0 coords(du: 4mm, dv: -1mm) holder
  b1 coords(du: 12mm, dv: -1mm) holder
  b2 coords(du: 12mm, dv: 1mm) holder
  b3 coords(du: 4mm, dv: 1mm) holder
  private lower := line(b0, b1)
  private end := line(b1, b2)
  private upper := line(b2, b3)
  private nose := line(b3, b0)
}
construction bar := solid(std.solids.Prism(face(lower, end, upper, nose), from: -8mm, to: 8mm))

// the frame the bar is carried by: turned t about z and risen 6 s + 0.3 s^2
carrier := frame
carrier.v parallel(sense: same) std.z
carrier.u perpendicular std.z
std.x angle(t, about: std.z) carrier.u
carrier.origin coincident std.z
carrier.origin distance(6mm * t / 360deg + 0.3mm * (t / 360deg)^2, along: n) std.top
construction thread := solid({ p | some t: p in bar at(under: carrier) t })
thread cut part
