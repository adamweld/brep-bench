// A thicker torus tumbled about nearly square axes: the torus centred at (2.02, 0, 1), its axis
// along z, its tube (radius 0.16) 1 from that axis, turned 1.01 t about A, the upright line through
// (2, 0, 0), and then -t about B, the line through the origin along u =
// (cos 89, sin 89 cos 1, sin 89 sin 1). The part is the kernel's sweep of the torus as t runs from
// -180 to 180 degrees, read as the torus turning about A seen from a frame turning +t about B.
unit mm
use std
use std.solids
use std.place (above, right_of)
use std.motion (turns)

// the torus: its tube's circle drawn in the front plane, 1 out from its upright axis through
// (2.02, 0, 1), turned about that axis
in std.front {
  private c := point
  fix((2.02, 1)) c
  private c_up := point
  c_up above(d: 1mm) c
  construction centerline torus_axis := line(c, c_up)
  private tc := point
  tc right_of(d: 1mm) c
  private tube := circle(center: tc) hint(r: 0.16)
  radius(0.16mm) tube
}
construction torus := solid(std.solids.Revolution(face(tube), about: torus_axis))

// A: the upright line through (2, 0, 0)
private a_foot := point
fix((2, 0, 0)) a_foot
private a := axis
fix(dir == (0, 0, 1)) a
a_foot coincident a
// B: the line through q = (0, 0, 0)
// along u = (cos 89, sin 89 cos 1, sin 89 sin 1)
private q := point
fix((0, 0, 0)) q
private b := axis
fix(dir == (cos(89deg), sin(89deg) * cos(1deg), sin(89deg) * sin(1deg))) b
q coincident b

// the torus turns 1.01 t about A while the frame it is seen from turns t about B, so seen from
// that frame it is turned 1.01 t about A and then -t about B
private spin := frame
spin turns(1.01) a
private observer := frame
observer turns(1) b
part := solid({ p | some t in (-180deg, 180deg): p in torus at(under: spin, seen_from: observer) t })
