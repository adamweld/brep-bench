// A straight bevel gear's tooth space cut by a tooth of its pinion turning in mesh with it: a
// 30-tooth gear about z and a 15-tooth pinion about x, module 2 at the outer end, the shafts
// meeting at the origin O. g = (2, 0, 1)/sqrt 5 runs along the line the pitch cones touch on and
// d = (-1, 0, 2)/sqrt 5 square to it, from the pinion into the gear.
//
// The blank is the given quadrilateral in the front plane (x, z) turned whole about z. The tooth
// is the pyramid with apex O over the trapezoid P + u d + w y (|u| <= 2.5,
// |w| <= pi/2 - u tan 20) in the plane square to g through P = 15 sqrt 5 g, cut off square to g
// at 15 sqrt 5 - 12 and 15 sqrt 5 + 2 from O: built as the join of its two end sections, each
// corner on the ray from O through the trapezoid's corner. The pinion turns t about x while the
// gear turns -t/2 about z; the kernel's sweep of the tooth as the gear sees it, t from -60 to 60
// degrees, is cut from the blank.
unit mm
use std
use std.solids (cut)
use std.sets
use std.place (coords)
use std.motion (turns)

cd := 15 * sqrt(5)   // the outer cone distance, where the pitch radius is 30
near := cd - 12      // where the tooth is cut off, toward O and away from it
far := cd + 2

// the blank, its section's corners as given
in std.front {
  private k0 := point
  private k1 := point
  private k2 := point
  private k3 := point
  fix((30.894427, 13.211146)) k0
  fix((27.316718, 20.366563)) k1
  fix((18.372447, 15.894427)) k2
  fix((21.683489, 9.272343)) k3
  private back := line(k0, k1)
  private bore := line(k1, k2)
  private front := line(k2, k3)
  private face_cone := line(k3, k0)
  construction centerline gear_axis := line(std.origin, hint((0, 1)))
  fix((0, 1)) gear_axis.p2
}
part := solid(std.solids.Revolution(face(back, bore, front, face_cone), about: gear_axis))

// the planes square to g (each its own axes, along d and y): through the outer pitch point, and
// at the tooth's two ends
private pitch_d := axis
fix(dir == (-1, 0, 2)) pitch_d
private pitch_y := axis
fix(dir == (0, 1, 0)) pitch_y
pitch := plane(u: pitch_d, v: pitch_y)
fix(origin == (2 * cd / sqrt(5), 0, cd / sqrt(5))) pitch
private inner_d := axis
fix(dir == (-1, 0, 2)) inner_d
private inner_y := axis
fix(dir == (0, 1, 0)) inner_y
inner := plane(u: inner_d, v: inner_y)
fix(origin == (2 * near / sqrt(5), 0, near / sqrt(5))) inner
private outer_d := axis
fix(dir == (-1, 0, 2)) outer_d
private outer_y := axis
fix(dir == (0, 1, 0)) outer_y
outer := plane(u: outer_d, v: outer_y)
fix(origin == (2 * far / sqrt(5), 0, far / sqrt(5))) outer

o := point
fix((0, 0, 0)) o
// the trapezoid at the pitch point, pi wide at the pitch cone, its sides 20 degrees from d
in pitch {
  private c0 := point
  private c1 := point
  private c2 := point
  private c3 := point
  c0 coords(du: 2.5mm, dv: (pi / 2 - 2.5 * tan(20deg)) * 1mm) pitch
  c1 coords(du: 2.5mm, dv: -(pi / 2 - 2.5 * tan(20deg)) * 1mm) pitch
  c2 coords(du: -2.5mm, dv: -(pi / 2 + 2.5 * tan(20deg)) * 1mm) pitch
  c3 coords(du: -2.5mm, dv: (pi / 2 + 2.5 * tan(20deg)) * 1mm) pitch
}
// its sections at the tooth's ends, each corner on the ray from O through the trapezoid's
in inner {
  private n0 := point hint((2, 1.1))
  private n1 := point hint((2, -1.1))
  private n2 := point hint((-2, -2.0))
  private n3 := point hint((-2, 2.0))
}
in outer {
  private f0 := point hint((2.7, 1.2))
  private f1 := point hint((2.7, -1.2))
  private f2 := point hint((-2.7, -2.4))
  private f3 := point hint((-2.7, 2.4))
}
n0 coincident line(o, c0)
n1 coincident line(o, c1)
n2 coincident line(o, c2)
n3 coincident line(o, c3)
f0 coincident line(o, c0)
f1 coincident line(o, c1)
f2 coincident line(o, c2)
f3 coincident line(o, c3)
in inner {
  small := face(n0, n1, n2, n3, -> close)
}
in outer {
  large := face(f0, f1, f2, f3, -> close)
}
construction tooth := solid({ p | some q in small, r in large: p in std.sets.Segment(q, r) })

// the pinion turning about x, the gear half as fast the other way about z
pinion := frame
pinion turns(1) std.x
gear := frame
gear turns(-1 / 2) gear_axis
construction space := solid({ p | some t in (-60deg, 60deg): p in tooth at(under: pinion, seen_from: gear) t })
space cut part
