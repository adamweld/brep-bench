// The solid a regular tetrahedron fills as it is carried along a screw. The tetrahedron, of edge
// 2√2, is centred 4 along x, and its three axes of symmetry run through its centre along x, y and
// z: one corner and its half turns about the three are its four corners, (5, 1, 1), (5, -1, -1),
// (3, 1, -1) and (3, -1, 1), its edges from that corner making it regular, and the seed picking
// the corner. It is the join of two opposite edges: every point of a segment between a point of
// the one and a point of the other. The screw is about z, right-handed, rising 5 a turn; the part
// is the kernel's sweep of the tetrahedron from a turn of 0 to 540 degrees.
unit mm
use std
use std.sets
use std.motion (screws)

centre := point hint((4, 0, 0))
centre coincident std.x
std.origin distance(4mm) centre
along_y := axis hint(dir: (0, 1, 0))
along_y parallel std.y
centre coincident along_y
along_z := axis hint(dir: (0, 0, 1))
along_z parallel std.z
centre coincident along_z

edge := 2mm * sqrt(2)
a := point hint((5, 1, 1))
b := point
c := point
d := point
a symmetry(std.x) b
a symmetry(along_y) c
a symmetry(along_z) d
a distance(edge) b
a distance(edge) c
a distance(edge) d
construction tetrahedron := solid({ p | some q in std.sets.Segment(a, b), r in std.sets.Segment(c, d):
  p in std.sets.Segment(q, r) })

feed := frame
feed screws(5mm) std.z
part := solid({ p | some t in (0deg, 540deg): p in tetrahedron at(under: feed) t })
