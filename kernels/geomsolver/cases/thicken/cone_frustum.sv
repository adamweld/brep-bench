// A conical sheet thickened by 1 along its normal away from the axis. The sheet is the developable
// panel between its two rims, the circles about the z axis of radius 6 at z = 0 and radius 3 at
// z = 4 (every ruling a generator of the cone); the thickened sheet is its band 1 thick on its
// outside, the side its normal a′ × (b − a) points to — out from the axis and leaning up, with the
// lower rim, run anticlockwise, given first. The band is measured along the normal, so its rims
// sweep cones square to the sheet.
unit mm
use std
use std.panels

in std.top {
  c0 := point
  fix((0mm, 0mm)) c0
  lower := circle(center: c0) hint(r: 6)
  radius(6mm) lower
}
top_plane := plane
fix(origin == (0mm, 0mm, 4mm)) top_plane
fix(dir == (1, 0, 0)) top_plane.u
fix(dir == (0, 1, 0)) top_plane.v
in top_plane {
  c1 := point
  fix((0mm, 0mm)) c1
  upper := circle(center: c1) hint(r: 3)
  radius(3mm) upper
}
sheet := std.panels.Developable(lower, upper)
part := solid({ q | q distance(<= 1mm, side: outside) sheet })
