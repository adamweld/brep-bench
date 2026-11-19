// A square-to-octagon transition: the join of the square -20 <= x, y <= 20 drawn in the top plane
// and the octagon drawn in the plane 30 above it, the 28 × 28 square with its corners cut off —
// every point of a segment from a point of one to a point of the other, their hull. Its walls are
// the flat trapezoids from each side of the square to the side of the octagon parallel to it, and
// the flat triangles from each corner of the square to the cut side facing it.
unit mm
use std
use std.sets

in std.top {
  a := point
  b := point hint((20, -20))
  c := point hint((20, 20))
  d := point hint((-20, 20))
  fix((-20, -20)) a
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
  horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  square := face(south, east, north, west)
  a distance(40mm) b
  b distance(40mm) c
}

// the octagon, its eight corners as the case gives them, in the plane z = 30
lid := plane
fix(origin == (0, 0, 30)) lid
fix(dir == (1, 0, 0)) lid.u
fix(dir == (0, 1, 0)) lid.v
in lid {
  k0 := point
  k1 := point
  k2 := point
  k3 := point
  k4 := point
  k5 := point
  k6 := point
  k7 := point
  fix((6, -14)) k0
  fix((14, -6)) k1
  fix((14, 6)) k2
  fix((6, 14)) k3
  fix((-6, 14)) k4
  fix((-14, 6)) k5
  fix((-14, -6)) k6
  fix((-6, -14)) k7
  octagon := face(k0, k1, k2, k3, k4, k5, k6, k7, -> close)
}

part := solid({ p | some q in square, r in octagon: p in std.sets.Segment(q, r) })
