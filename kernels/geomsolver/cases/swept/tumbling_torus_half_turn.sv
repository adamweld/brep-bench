// An oblique torus tumble through half a turn: the torus centred at (3, 0, 2), its axis along z,
// its tube (radius 0.16) 1 from that axis, turned 0.375 t about A, the upright line through
// (2, 0, 0), and then -t about B, the line through q = (0, h cos 30, h sin 30), h = 0.7071, along u
// = (cos 20, sin 20 cos 30, sin 20 sin 30). The part is the kernel's sweep of the torus as t runs
// from -90 to 90 degrees, read as the torus turning about A seen from a frame turning +t about B.
unit mm
use std
use std.solids
use std.place (above, right_of)
use std.motion (turns)

// the torus: its tube's circle drawn in the front plane, 1 out from its upright axis through
// (3, 0, 2), turned about that axis
in std.front {
  private c := point
  fix((3, 2)) c
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
// B: the line through q = (0, 0.7071 cos 30, 0.7071 sin 30)
// along u = (cos 20, sin 20 cos 30, sin 20 sin 30)
private q := point
fix((0, 0.7071 * cos(30deg), 0.7071 * sin(30deg))) q
private b := axis
fix(dir == (cos(20deg), sin(20deg) * cos(30deg), sin(20deg) * sin(30deg))) b
q coincident b

// the torus turns 0.375 t about A while the frame it is seen from turns t about B, so seen from
// that frame it is turned 0.375 t about A and then -t about B
private spin := frame
spin turns(0.375) a
private observer := frame
observer turns(1) b
part := solid({ p | some t in (-90deg, 90deg): p in torus at(under: spin, seen_from: observer) t })
