// A rod threaded partway up by a square bar that is then withdrawn: a thread with a run-out. The
// rod is the cylinder of radius 5 about z from z = 0 to 20. The bar is the box 4 <= x <= 12,
// -8 <= y <= 8, -1 <= z <= 1 as placed: its section in a plane through the rod's axis and the
// prism over it 8 either side. Its path is two pieces, a screw (6 a turn, from a turn of -90 to
// 900 degrees) and then a slide 3 along -x, its own length, so the motion is a frame the drawing
// moves: with one parameter t from -90 to 990 degrees, the frame `carrier` stands turned
// min(t, 900deg) about z, risen 6 a turn of that, and slid out along -x 3 for the last 90 degrees
// of t. The kernel's sweep of the bar under it is cut from the rod. Such a frame carries what is
// drawn from where it stands as drawn, at the parameter's first value, so the bar is drawn where
// it starts.
unit mm
use std
use std.solids (cut)
use std.place (coords)

param t: Angle over (-90deg, 990deg)

in std.top {
  private o := point
  fix((0, 0)) o
  private rim := circle(center: o) hint(r: 5)
  radius(5mm) rim
}
part := solid(std.solids.Prism(face(rim), from: 0mm, to: 20mm))

// the bar drawn where it starts, t = -90deg: turned -90deg about z, its middle 1.5 down
private start_u := axis
fix(dir == (0, -1, 0)) start_u
private start_v := axis
fix(dir == (0, 0, 1)) start_v
holder := plane(u: start_u, v: start_v)
fix(origin == (0, 0, -1.5)) holder
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

// the frame the bar is carried by: turned min(t, 900deg) about z (the screw, held where it
// stops), risen 6 a turn of that, and slid out along -x 3 over the last 90 degrees of t
carrier := frame
carrier.v parallel(sense: same) std.z
carrier.u perpendicular std.z
std.x angle(min(t, 900deg), about: std.z) carrier.u
carrier.origin distance(0mm, along: n) std.front
carrier.origin distance(-3mm * max(0deg, t - 900deg) / 90deg, along: n) std.side
carrier.origin distance(6mm * min(t, 900deg) / 360deg, along: n) std.top
construction thread := solid({ p | some t: p in bar at(under: carrier) t })
thread cut part
