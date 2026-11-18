// The regularized difference of two torus tumbles, each swept over its own roll. The torus is
// centred at (3, 0, 2), its axis along z, its tube (radius 0.22) 1 from that axis. Sweep A turns it
// 0.7 t about the upright line through (2, 0, 0) and then -t about B, the line through q =
// (0, h cos 45, h sin 45), h = 0.7071, along u = (cos 20, sin 20 cos 45, sin 20 sin 45); sweep B
// the same but +t about B; t runs -100 to 100 degrees in each. Each is the kernel's sweep of the
// torus turning about A seen from a frame turning about B; the part is A less B.
unit mm
use std
use std.solids (cut)
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
  private tube := circle(center: tc) hint(r: 0.22)
  radius(0.22mm) tube
}
construction torus := solid(std.solids.Revolution(face(tube), about: torus_axis))

// A: the upright line through (2, 0, 0)
private a_foot := point
fix((2, 0, 0)) a_foot
private a := axis
fix(dir == (0, 0, 1)) a
a_foot coincident a
// B: the line through q = (0, 0.7071 cos 45, 0.7071 sin 45)
// along u = (cos 20, sin 20 cos 45, sin 20 sin 45)
private q := point
fix((0, 0.7071 * cos(45deg), 0.7071 * sin(45deg))) q
private b := axis
fix(dir == (cos(20deg), sin(20deg) * cos(45deg), sin(20deg) * sin(45deg))) b
q coincident b

private spin_fwd := frame
spin_fwd turns(0.7) a
private observer_fwd := frame
observer_fwd turns(1) b
private spin_rev := frame
spin_rev turns(0.7) a
private observer_rev := frame
observer_rev turns(-1) b
construction reverse := solid({ p | some t in (-100deg, 100deg): p in torus at(under: spin_rev, seen_from: observer_rev) t })
part := solid({ p | some t in (-100deg, 100deg): p in torus at(under: spin_fwd, seen_from: observer_fwd) t })
reverse cut part
