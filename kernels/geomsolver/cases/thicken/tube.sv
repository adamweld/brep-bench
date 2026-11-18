// A cylindrical sheet thickened by 1 away from its axis into a tube. The sheet is the developable
// panel between its two rims, the circles of radius 5 about the z axis at z = 0 and z = 10 (every
// ruling a generator, the band closing round on itself with no seam wall); the thickened sheet is
// its band 1 thick on its outside, the side its normal a′ × (b − a) points to: away from the axis
// for the lower rim, run anticlockwise, given first.
unit mm
use std
use std.panels

in std.top {
  c0 := point
  fix((0mm, 0mm)) c0
  lower := circle(center: c0) hint(r: 5)
  radius(5mm) lower
}
top_plane := plane
fix(origin == (0mm, 0mm, 10mm)) top_plane
fix(dir == (1, 0, 0)) top_plane.u
fix(dir == (0, 1, 0)) top_plane.v
in top_plane {
  c1 := point
  fix((0mm, 0mm)) c1
  upper := circle(center: c1) hint(r: 5)
  radius(5mm) upper
}
sheet := std.panels.Developable(lower, upper)
part := solid({ q | q distance(<= 1mm, side: outside) sheet })
