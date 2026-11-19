// A spheroid turned from a weighted quadratic B-spline: in the front plane (x, z), the rational
// quadratic B-spline on P0 = (0, -10), P1 = (6, -10), P2 = (6, 0), P3 = (6, 10) and P4 = (0, 10),
// weighing 1, √2/2, 1, √2/2 and 1, with a double knot at its middle, closed by the z axis, turned a
// whole turn about the z axis. A spline here is a cubic, so each of its two quadratic pieces is
// raised to one exactly — its inner control points the fraction 2w / (1 + 2w) = 2 - √2 of the
// way from each end toward the middle point, each weighing (1 + 2w) / 3, w = √2/2 — and the two
// cubics joined at P2 by a triple knot.
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
  fix((0, -10)) p0
  fix((6, -10)) p1
  fix((6, 0)) p2
  fix((6, 10)) p3
  fix((0, 10)) p4
  a := point
  b := point
  c := point
  d := point
  construction leg0 := line(p0, p1)
  construction leg1 := line(p2, p1)
  construction leg2 := line(p2, p3)
  construction leg3 := line(p4, p3)
  a coincident(t == 2 - sqrt(2)) leg0
  b coincident(t == 2 - sqrt(2)) leg1
  c coincident(t == 2 - sqrt(2)) leg2
  d coincident(t == 2 - sqrt(2)) leg3
  wall := spline(p0, a, b, p2, c, d, p4) knots [0, 0, 0, 0, 0.5, 0.5, 0.5, 1, 1, 1, 1] weights [1, heavy, heavy, 1, heavy, heavy, 1]
  construction centerline spindle := line(p4, p0)
}

part := solid(std.solids.Revolution(face(wall, spindle), about: spindle))
