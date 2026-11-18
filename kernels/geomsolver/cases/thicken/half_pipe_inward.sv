// A half-pipe sheet thickened by 2 toward its axis. The sheet is the developable panel between
// its two half-circle edges, radius 5 about the z axis where y ≥ 0, at z = 0 and z = 10, each run
// anticlockwise from (5, 0) to (−5, 0); the thickened sheet is its band 2 thick on its inside, the
// side its normal a′ × (b − a) points away from, toward the axis with the lower edge given first.
// The band is square at the sheet's straight edges, so its ends there are the flat strips in y = 0.
unit mm
use std
use std.panels

in std.top {
  o0 := point
  fix((0mm, 0mm)) o0
  s0 := point
  fix((5mm, 0mm)) s0
  e0 := point hint((-5, 0))
  o0 distance(0mm, along: v) e0
  lower := arc(center: o0, start: s0, end: e0) hint(r: 5)
}
top_plane := plane
fix(origin == (0mm, 0mm, 10mm)) top_plane
fix(dir == (1, 0, 0)) top_plane.u
fix(dir == (0, 1, 0)) top_plane.v
in top_plane {
  o1 := point
  fix((0mm, 0mm)) o1
  s1 := point
  fix((5mm, 0mm)) s1
  e1 := point hint((-5, 0))
  o1 distance(0mm, along: v) e1
  upper := arc(center: o1, start: s1, end: e1) hint(r: 5)
}
sheet := std.panels.Developable(lower, upper)
part := solid({ q | q distance(<= 2mm, side: inside) sheet })
