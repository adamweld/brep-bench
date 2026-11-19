// A drum united with a sleeve whose bore is a NURBS cylinder. The drum is the circle of radius 5
// about the origin of the top plane and the prism over it from z = 0 to z = 10. The sleeve is the
// prism from z = 2 to z = 8 over the ring between the circle of radius 7 and the bore: the
// rational quadratic circle of radius 5 on the nine-point net (0, 5), (-5, 5), (-5, 0), (-5, -5),
// (0, -5), (5, -5), (5, 0), (5, 5), (0, 5) weighing 1, √2/2, 1, …, its seam at (0, 5), slid
// straight along z (degree 1 along the axis). A spline here is a cubic, so each quarter of the
// bore, a rational quadratic on three points of the net, is raised to a cubic exactly: its inner
// control points the fraction 2 - √2 of the way from each end toward the middle point, each
// weighing (1 + √2) / 3.
unit mm
use std
use std.solids
use std.solids (union)

heavy := (1 + sqrt(2)) / 3

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5mm) rim
  outside := circle(center: o) hint(r: 7)
  radius(7mm) outside
  c0 := point
  c1 := point
  c2 := point
  c3 := point
  c4 := point
  c5 := point
  c6 := point
  c7 := point
  fix((0, 5)) c0
  fix((-5, 5)) c1
  fix((-5, 0)) c2
  fix((-5, -5)) c3
  fix((0, -5)) c4
  fix((5, -5)) c5
  fix((5, 0)) c6
  fix((5, 5)) c7
  // each quarter's inner control points, on the legs of the net either side of its middle point
  construction l0 := line(c0, c1)
  construction l1 := line(c2, c1)
  construction l2 := line(c2, c3)
  construction l3 := line(c4, c3)
  construction l4 := line(c4, c5)
  construction l5 := line(c6, c5)
  construction l6 := line(c6, c7)
  construction l7 := line(c0, c7)
  k0 := point
  k1 := point
  k2 := point
  k3 := point
  k4 := point
  k5 := point
  k6 := point
  k7 := point
  k0 coincident(t == 2 - sqrt(2)) l0
  k1 coincident(t == 2 - sqrt(2)) l1
  k2 coincident(t == 2 - sqrt(2)) l2
  k3 coincident(t == 2 - sqrt(2)) l3
  k4 coincident(t == 2 - sqrt(2)) l4
  k5 coincident(t == 2 - sqrt(2)) l5
  k6 coincident(t == 2 - sqrt(2)) l6
  k7 coincident(t == 2 - sqrt(2)) l7
  nw := spline(c0, k0, k1, c2) weights [1, heavy, heavy, 1]
  sw := spline(c2, k2, k3, c4) weights [1, heavy, heavy, 1]
  se := spline(c4, k4, k5, c6) weights [1, heavy, heavy, 1]
  ne := spline(c6, k6, k7, c0) weights [1, heavy, heavy, 1]
  bore := face(nw, sw, se, ne)
}

drum := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
sleeve := solid(std.solids.Prism(face(outside, holes: bore), from: 2mm, to: 8mm))
part := solid(drum)
sleeve union part
