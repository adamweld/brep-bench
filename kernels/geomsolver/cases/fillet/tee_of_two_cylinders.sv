// A tee of two solid cylinders: the bar, radius 6 about the x axis from x = −20 to 20, its circle
// drawn in the side plane (y, z); and the stem, radius 3 about the z axis from z = 0 to 15, its
// circle drawn in the top plane, in union with it. The saddle where the stem's faces meet the
// bar's is rounded to radius 1.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds (round)

in std.side {
  c1 := point
  fix((0, 0)) c1
  bar_k := circle(center: c1) hint(r: 6)
  radius(6mm) bar_k
}
in std.top {
  c2 := point
  fix((0, 0)) c2
  stem_k := circle(center: c2) hint(r: 3)
  radius(3mm) stem_k
}

bar := solid(std.solids.Prism(face(bar_k), from: -20mm, to: 20mm))
stem := solid(std.solids.Prism(face(stem_k), from: 0mm, to: 15mm))
part := solid(bar)
stem union part
part.stem round(r: 1mm) part.bar
