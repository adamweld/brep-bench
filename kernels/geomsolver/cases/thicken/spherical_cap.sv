// A spherical cap thickened by 2 outward. The sheet is the cap of the sphere of radius 10 about the
// origin where z ≥ 6, bounded by the circle of radius 8 at z = 6; thickened away from the center
// every point moves along the radius through it, so the thickened sheet is the points within 2 of
// the ball and not in it that lie in the cone the rim's radii sweep — apex at the origin, about the
// z axis, its half angle α with cos α = 3/5. (Solvent's thickening of a sheet, a developable
// panel's band, takes a ruled panel between two rails; a sphere is neither, so the sheet is
// thickened as the offset of the region it bounds, kept within the cone its rim sweeps.)
unit mm
use std
use std.sets

o := point
fix((0, 0, 0)) o
up := point
fix((0, 0, 1)) up
ball := std.sets.Ball(o, r: 10mm)
z_axis := line(o, up)
rim_cone := std.sets.Cone(z_axis, half: acos(3 / 5))
part := solid({ q | q distance(<= 2mm) ball; q not in ball; q in rim_cone })
