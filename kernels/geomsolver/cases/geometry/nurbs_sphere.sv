// A ball bounded by one NURBS surface with two poles. The surface is the rational quadratic
// meridian, the semicircle on (0, -5), (5, -5), (5, 0), (5, 5), (0, 5) weighing 1, √2/2, 1, √2/2, 1
// with a double knot at its middle, carried round the axis; Solvent has no surface patch, so it
// is the meridian, drawn in the front plane (x, z) — its seam in the half plane y = 0, x >= 0 —
// turned a whole turn about the z axis, closed by the axis, its poles at its ends. A spline here
// is a cubic, so each of the meridian's two quadratic pieces is raised to one exactly — its inner
// control points the fraction 2w / (1 + 2w) = 2 - √2 of the way from each end toward the middle
// point, each weighing (1 + 2w) / 3, w = √2/2 — and the two joined at (5, 0) by a triple knot.
unit mm
use std
use std.solids

heavy := (1 + sqrt(2)) / 3

in std.front {
  p0 := point
  p1 := point
  p2 := point
  p3 := point
  p4 := point
  fix((0, -5)) p0
  fix((5, -5)) p1
  fix((5, 0)) p2
  fix((5, 5)) p3
  fix((0, 5)) p4
  construction leg0 := line(p0, p1)
  construction leg1 := line(p2, p1)
  construction leg2 := line(p2, p3)
  construction leg3 := line(p4, p3)
  a := point
  b := point
  c := point
  d := point
  a coincident(t == 2 - sqrt(2)) leg0
  b coincident(t == 2 - sqrt(2)) leg1
  c coincident(t == 2 - sqrt(2)) leg2
  d coincident(t == 2 - sqrt(2)) leg3
  meridian := spline(p0, a, b, p2, c, d, p4) knots [0, 0, 0, 0, 0.5, 0.5, 0.5, 1, 1, 1, 1] weights [1, heavy, heavy, 1, heavy, heavy, 1]
  construction centerline spindle := line(p4, p0)
}

part := solid(std.solids.Revolution(face(meridian, spindle), about: spindle))
