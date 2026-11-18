// A shaft, the circle of radius 5 about the origin drawn in the top plane and its prism from
// z = 0 to z = 20, with a V-groove turned into it: everything farther from the axis than
// 3 + |z − 10| cut away, by the turn about the z axis of the triangle drawn in the front plane
// (x, z) through (3, 10), (7, 6) and (7, 14), its two flanks running out at 45° past the shaft.
// Then both of the groove's cones deleted together.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit is
// said as one: deleting the groove runs the faces round it on along their own surfaces until they
// meet, so what is left is the points on the material's side of the surface of every face of the
// grooved shaft but the groove's (its two ends and its cylinder, below the groove and above it),
// read from its own faces.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim
}
shaft := solid(std.solids.Prism(face(rim), from: 0mm, to: 20mm))

in std.front {
  v0 := point hint((3, 10))
  v1 := point hint((7, 6))
  v2 := point hint((7, 14))
  (lower := line(v0, v1)) -> (outer := line(v1, v2)) -> (upper := line(v2, v0)) -> close
  fix((3, 10)) v0
  fix((7, 6)) v1
  fix((7, 14)) v2
}
groove := solid(std.solids.Revolution(face(lower, outer, upper), about: std.z))
groove cut shaft

part := solid({ p | p in shaft.near.side; p in shaft.far.side; p in shaft.rim.side })
