// A hemispherical sheet thickened by 5, its whole radius, toward the sphere's center. The sheet is
// the upper half of the sphere of radius 5 about the origin, the boundary of the ball where z ≥ 0;
// thickened inward by 5 it is the points of the ball within 5 of what is outside it, above the top
// plane: the half ball, its flat face the disk the equator sweeps as it closes on the center.
// (Solvent's thickening of a sheet, a developable panel's band, takes a ruled panel between two
// rails; a sphere is neither, so the sheet is thickened as an offset of the region it bounds.)
unit mm
use std
use std.sets

o := point
fix((0, 0, 0)) o
ball := std.sets.Ball(o, r: 5mm)
part := solid({ q |
  q in ball
  q distance(<= 5mm) { p | p not in ball }
  q distance(>= 0mm, along: n) std.top
})
