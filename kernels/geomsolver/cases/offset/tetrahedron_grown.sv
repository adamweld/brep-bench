// The regular tetrahedron on the corners (3, 3, 3), (3, −3, −3), (−3, 3, −3) and (−3, −3, 3), grown
// by 1. The tetrahedron is the hull of its corners, written as the join of two opposite edges: the
// segments from some point of one edge to some point of the other. Grown, it is the points a ball
// of radius 1 about some point of it reaches.
unit mm
use std
use std.sets

a := point
b := point
c := point
d := point
fix((3, 3, 3)) a
fix((3, -3, -3)) b
fix((-3, 3, -3)) c
fix((-3, -3, 3)) d

tetrahedron := solid({ p | some q in std.sets.Segment(a, b), r in std.sets.Segment(c, d):
  p in std.sets.Segment(q, r) })
part := solid({ p | some q in tetrahedron: p distance(<= 1mm) q })
