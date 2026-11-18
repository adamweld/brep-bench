// The solid a regular tetrahedron fills as it is carried along a screw. The tetrahedron is the
// join of two opposite edges, the segments (5, 1, 1)-(5, -1, -1) and (3, 1, -1)-(3, -1, 1): every
// point of a segment between a point of the one and a point of the other. The screw is about z,
// right-handed, rising 5 a turn; the part is the kernel's sweep of the tetrahedron from a turn of
// 0 to 540 degrees.
unit mm
use std
use std.sets
use std.motion (screws)

a := point
b := point
c := point
d := point
fix((5, 1, 1)) a
fix((5, -1, -1)) b
fix((3, 1, -1)) c
fix((3, -1, 1)) d
construction tetrahedron := solid({ p | some q in std.sets.Segment(a, b), r in std.sets.Segment(c, d):
  p in std.sets.Segment(q, r) })

feed := frame
feed screws(5mm) std.z
part := solid({ p | some t in (0deg, 540deg): p in tetrahedron at(under: feed) t })
