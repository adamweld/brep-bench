// A ring turned from a periodic quadratic B-spline: the closed uniform quadratic B-spline on the
// control square (13, 3), (7, 3), (7, -3), (13, -3) of the front plane (x, z) is four parabolic
// arcs, each from the middle of one side of the square to the middle of the next with the corner
// between them as its control point. Solvent has no periodic spline and its splines are cubics, so
// each arc is a quadratic raised to a cubic exactly — its inner control points two thirds of the
// way from each end toward the corner — and the four bound the section, turned a whole turn about
// the z axis.
unit mm
use std
use std.solids

// a parabolic arc from `a` to `b` with the control point `c`, raised to a cubic
component Arc(a: point, c: point, b: point) {
  in c.plane {
    construction near := line(a, c)
    construction far := line(b, c)
    k1 := point
    k2 := point
    k1 coincident(t == 2 / 3) near
    k2 coincident(t == 2 / 3) far
    g := spline(a, k1, k2, b)
  }
}

in std.front {
  c0 := point
  c1 := point
  c2 := point
  c3 := point
  fix((13, 3)) c0
  fix((7, 3)) c1
  fix((7, -3)) c2
  fix((13, -3)) c3
  construction side0 := line(c3, c0)
  construction side1 := line(c0, c1)
  construction side2 := line(c1, c2)
  construction side3 := line(c2, c3)
  m0 := point
  m1 := point
  m2 := point
  m3 := point
  m0 midpoint side0
  m1 midpoint side1
  m2 midpoint side2
  m3 midpoint side3
}
a0 := Arc(m0, c0, m1)
a1 := Arc(m1, c1, m2)
a2 := Arc(m2, c2, m3)
a3 := Arc(m3, c3, m0)

part := solid(std.solids.Revolution(face(a0.g, a1.g, a2.g, a3.g), about: std.z))
