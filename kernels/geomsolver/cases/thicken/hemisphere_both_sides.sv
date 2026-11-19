// A hemispherical sheet thickened by 1 to each side. The sheet is the upper half of the sphere of
// radius 5 about the origin, the boundary of the ball where z ≥ 0; thickened to both sides it is the
// points within 1 of the ball and within 1 of what is outside it — within 1 of the sphere — above
// the top plane, square to the sheet at its rim. (Solvent's thickening of a sheet, a developable
// panel's band, takes a ruled panel between two rails; a sphere is neither, so the sheet is
// thickened as offsets of the region it bounds.)
unit mm
use std
use std.sets

o := point
fix((0, 0, 0)) o
ball := std.sets.Ball(o, r: 5mm)
part := solid({ q |
  q distance(<= 1mm) ball
  q distance(<= 1mm) { p | p not in ball }
  q distance(>= 0mm, along: n) std.top
})
