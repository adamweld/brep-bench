// A closed spherical sheet thickened by 1 outward into a hollow ball. The sheet is the sphere of
// radius 5 about the origin, the boundary of the ball it closes round; thickened outward it is the
// points within 1 of that ball and not in it. (Solvent's thickening of a sheet, a developable
// panel's band, takes a ruled panel between two rails; a sphere is neither, so the sheet is
// thickened as the offset of the region it bounds.)
unit mm
use std
use std.sets

o := point
fix((0, 0, 0)) o
ball := std.sets.Ball(o, r: 5mm)
part := solid({ q | q distance(<= 1mm) ball; q not in ball })
