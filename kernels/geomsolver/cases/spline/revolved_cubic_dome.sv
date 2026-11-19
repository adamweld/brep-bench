// A dome turned from a cubic Bézier: in the front plane (x, z), the base from the origin out to
// (9, 0), the Bézier on the control points (9, 0), (12, 4), (9, 10) and (0, 10), and the z axis
// back down, turned a whole turn about the z axis.
unit mm
use std
use std.solids

in std.front {
  o := point
  k0 := point
  k1 := point
  k2 := point
  k3 := point
  fix((0, 0)) o
  fix((9, 0)) k0
  fix((12, 4)) k1
  fix((9, 10)) k2
  fix((0, 10)) k3
  base := line(o, k0)
  wall := spline(k0, k1, k2, k3)
  construction centerline spindle := line(k3, o)
}

part := solid(std.solids.Revolution(face(base, wall, spindle), about: spindle))
