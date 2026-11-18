// A block capped by the Coons patch of four straight edges, built as the case allows, as the ruled
// surface it also is: the block's two ends, the rectangle 10 × 5 in the side plane x = 0 and the
// trapezoid rising from 5 to 15 in the plane x = 10, each drawn in its plane, lofted by
// `std.stations.Ruled`, corner to corner by straight rails along x. Every section is the plane
// x = const, cut by the rails in a quadrilateral whose top edge is a line of the saddle
// z = 5 + xy/10, so the loft's top wall is the patch and its other walls the floor and the sides.
unit mm
use std
use std.stations

far_end := plane
fix(origin == (10mm, 0mm, 0mm)) far_end
fix(dir == (0, 1, 0)) far_end.u
fix(dir == (0, 0, 1)) far_end.v

// in each end plane, u is y and v is z
in std.side {
  a0 := point
  a1 := point
  a2 := point
  a3 := point
  fix((0mm, 0mm)) a0
  fix((10mm, 0mm)) a1
  fix((10mm, 5mm)) a2
  fix((0mm, 5mm)) a3
}
in far_end {
  b0 := point
  b1 := point
  b2 := point
  b3 := point
  fix((0mm, 0mm)) b0
  fix((10mm, 0mm)) b1
  fix((10mm, 15mm)) b2
  fix((0mm, 5mm)) b3
}
near_face := face(a0, a1, a2, a3, -> close)
far_face := face(b0, b1, b2, b3, -> close)
param s: Scalar over (0, 1)
part := solid({ p | some s: p in std.stations.Ruled(near_face, far_face).f at s })
