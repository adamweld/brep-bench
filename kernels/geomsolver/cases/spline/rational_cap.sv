// A ball's cap turned from a weighted quadratic arc: in the front plane (x, z), the floor from
// (0, -5) out to (5√3, -5), the rational quadratic Bézier on P0 = (5√3, -5), P1 = (10√3, 10) and
// P2 = (0, 10) weighing 1, 1/2 and 1, and the z axis back down, turned a whole turn about the
// z axis. A spline here is a cubic, so the quadratic is raised to one exactly: its inner control
// points (P0 + 2w P1) / (1 + 2w) and (2w P1 + P2) / (1 + 2w), with w = 1/2 the midpoints of its
// two legs, each weighing (1 + 2w) / 3 = 2/3.
unit mm
use std
use std.solids

in std.front {
  foot := point
  p0 := point
  p1 := point
  p2 := point
  fix((0, -5)) foot
  fix((5 * sqrt(3), -5)) p0
  fix((10 * sqrt(3), 10)) p1
  fix((0, 10)) p2
  construction leg0 := line(p0, p1)
  construction leg1 := line(p1, p2)
  m0 := point
  m1 := point
  m0 midpoint leg0
  m1 midpoint leg1
  floor := line(foot, p0)
  arch := spline(p0, m0, m1, p2) weights [1, 2 / 3, 2 / 3, 1]
  construction centerline spindle := line(p2, foot)
}

part := solid(std.solids.Revolution(face(floor, arch, spindle), about: spindle))
