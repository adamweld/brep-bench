// The regular tetrahedron on the corners (3, 3, 3), (3, −3, −3), (−3, 3, −3) and (−3, −3, 3), grown
// by 1. Its three axes of symmetry are the world's: one corner and its half turns about x, y and z
// are its four, and its three edges from that corner, 6√2 long, make it regular. The seed picks
// the corner (3, 3, 3) of the eight its relations allow. It is the hull of its corners, written as
// the join of two opposite edges: the segments from some point of one edge to some point of the
// other. Grown, it is the points a ball of radius 1 about some point of it reaches.
unit mm
use std
use std.sets

edge := 6mm * sqrt(2)
a := point hint((1, 1, 1))
b := point
c := point
d := point
a symmetry(std.x) b
a symmetry(std.y) c
a symmetry(std.z) d
a distance(edge) b
a distance(edge) c
a distance(edge) d

tetrahedron := solid({ p | some q in std.sets.Segment(a, b), r in std.sets.Segment(c, d):
  p in std.sets.Segment(q, r) })
part := solid({ p | some q in tetrahedron: p distance(<= 1mm) q })
