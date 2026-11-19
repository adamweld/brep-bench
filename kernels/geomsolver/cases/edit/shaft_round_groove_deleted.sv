// A shaft, the circle of radius 5 about the origin drawn in the top plane and its prism from
// z = 0 to z = 20, with a round-bottomed groove turned into it: the torus whose tube, the circle
// of radius 1.5 about (5, 10) drawn in the front plane (x, z), is turned about the z axis, cut
// away. Then the groove's face deleted.
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
  k := point
  fix((5, 10)) k
  tube := circle(center: k) hint(r: 1.5)
  radius(1.5) tube
}
groove := solid(std.solids.Revolution(face(tube), about: std.z))
groove cut shaft

part := solid({ p | p in shaft.near.side; p in shaft.far.side; p in shaft.rim.side })
