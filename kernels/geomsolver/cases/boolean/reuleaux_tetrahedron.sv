// A Reuleaux tetrahedron. The four centres are a regular tetrahedron of side 10, said as six
// distances: the first at the origin, the second 10 along x, the third in the top plane on the +y
// side, the fourth above it. The part is the ball of radius 10 about the first, kept where it is
// in common with each of the other three.
unit mm
use std
use std.solids (common)
use std.sets

a := point
fix((0, 0, 0)) a
b := point
fix((10, 0, 0)) b
c := point hint((5, 8.7, 0))
c coincident std.top
d := point hint((5, 2.9, 8.2))
a distance(10mm) c
b distance(10mm) c
a distance(10mm) d
b distance(10mm) d
c distance(10mm) d

part := solid(std.sets.Ball(a, r: 10mm))
repeat k in [b, c, d] {
  ball := solid(std.sets.Ball(k, r: 10mm))
  ball common part
}
