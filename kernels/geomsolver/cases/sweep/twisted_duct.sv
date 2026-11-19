// A square duct turned to a diamond: along the y axis from y = 0 to y = 40, the section at each
// station the quadrilateral whose corners each run straight from a corner of the square at the
// start, A (10, 0, -10), B (-10, 0, -10), C (-10, 0, 10), D (10, 0, 10), to the like corner of the
// diamond at the end, A' (0, 40, -12), B' (-12, 40, 0), C' (0, 40, 12), D' (12, 40, 0). The four
// ends are drawn as faces and lofted by straight rails, corner to corner, so each long side is the
// ruled face between two rails.
unit mm
use std
use std.stations

// the square at the start, in the front plane (x, z) at y = 0
in std.front {
  a := point
  b := point
  c := point
  d := point
  fix((10, -10)) a
  fix((-10, -10)) b
  fix((-10, 10)) c
  fix((10, 10)) d
  square := face(a, b, c, d, -> close)
}

// the diamond at the end, in the plane 40 along y
far := plane
fix(origin == (0, 40, 0)) far
fix(dir == (1, 0, 0)) far.u
fix(dir == (0, 0, 1)) far.v
in far {
  a2 := point
  b2 := point
  c2 := point
  d2 := point
  fix((0, -12)) a2
  fix((-12, 0)) b2
  fix((0, 12)) c2
  fix((12, 0)) d2
  diamond := face(a2, b2, c2, d2, -> close)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in std.stations.Ruled(square, diamond).f at s })
