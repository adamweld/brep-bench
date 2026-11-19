// A rod threaded by a bar whose section is a closed spline, carried along a screw. The rod is the
// cylinder of radius 5 about z from z = 0 to 20. The bar's section, in the front plane (x, z), is
// the closed uniform cubic B-spline over the seven given control points; Solvent's splines are
// clamped, so the closed curve is written as its seven spans, each the cubic Bezier whose ends are
// the blend (k[i-1] + 4 k[i] + k[i+1]) / 6 and whose inner control points lie a third and two
// thirds along the leg k[i] k[i+1] - the same curve, span for span. The bar is that section
// carried from y = -8 to 8. The screw is about z, right-handed, rising 6 a turn; the kernel's
// sweep of the bar from a turn of -90 to 1350 degrees is cut from the rod.
unit mm
use std
use std.solids (cut)
use std.motion (screws)

// where a uniform cubic B-spline's span starts, (a + 4 b + c) / 6: a third of the way from `b`
// to the middle of `a` and `c`
component Joint(a: point, b: point, c: point) {
  construction chord := line(a, c)
  private mid := point hint(((a.x + c.x) / 2, (a.y + c.y) / 2))
  mid coincident(t == 0.5) chord
  construction lean := line(b, mid)
  p := point hint(((a.x + 4 * b.x + c.x) / 6, (a.y + 4 * b.y + c.y) / 6))
  p coincident(t == 1 / 3) lean
}
// a span's inner control point: a third of the way from `a` to `b`
component Third(a: point, b: point) {
  construction leg := line(a, b)
  p := point hint((a.x + (b.x - a.x) / 3, a.y + (b.y - a.y) / 3))
  p coincident(t == 1 / 3) leg
}

in std.top {
  private o := point
  fix((0, 0)) o
  private rim := circle(center: o) hint(r: 5)
  radius(5mm) rim
}
part := solid(std.solids.Prism(face(rim), from: 0mm, to: 20mm))

in std.front {
  private k0 := point
  private k1 := point
  private k2 := point
  private k3 := point
  private k4 := point
  private k5 := point
  private k6 := point
  fix((3.5, 0)) k0
  fix((4.5, -1.5)) k1
  fix((8, -1.5)) k2
  fix((12, -1)) k3
  fix((12, 1)) k4
  fix((8, 1.5)) k5
  fix((4.5, 1.5)) k6
  // each span's ends, where the blend of three consecutive control points puts them
  private j0 := Joint(k6, k0, k1)
  private j1 := Joint(k0, k1, k2)
  private j2 := Joint(k1, k2, k3)
  private j3 := Joint(k2, k3, k4)
  private j4 := Joint(k3, k4, k5)
  private j5 := Joint(k4, k5, k6)
  private j6 := Joint(k5, k6, k0)
  // and its inner control points, a third and two thirds of the way along the leg k[i]-k[i+1]
  private m0 := Third(k0, k1)
  private q0 := Third(k1, k0)
  private m1 := Third(k1, k2)
  private q1 := Third(k2, k1)
  private m2 := Third(k2, k3)
  private q2 := Third(k3, k2)
  private m3 := Third(k3, k4)
  private q3 := Third(k4, k3)
  private m4 := Third(k4, k5)
  private q4 := Third(k5, k4)
  private m5 := Third(k5, k6)
  private q5 := Third(k6, k5)
  private m6 := Third(k6, k0)
  private q6 := Third(k0, k6)
  private s0 := spline(j0.p, m0.p, q0.p, j1.p)
  private s1 := spline(j1.p, m1.p, q1.p, j2.p)
  private s2 := spline(j2.p, m2.p, q2.p, j3.p)
  private s3 := spline(j3.p, m3.p, q3.p, j4.p)
  private s4 := spline(j4.p, m4.p, q4.p, j5.p)
  private s5 := spline(j5.p, m5.p, q5.p, j6.p)
  private s6 := spline(j6.p, m6.p, q6.p, j0.p)
}
construction bar := solid(std.solids.Prism(face(s0, s1, s2, s3, s4, s5, s6), from: -8mm, to: 8mm))

feed := frame
feed screws(6mm) std.z
construction thread := solid({ p | some t in (-90deg, 1350deg): p in bar at(under: feed) t })
thread cut part
