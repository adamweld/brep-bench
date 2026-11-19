// A plate, the circle of radius 8 about the origin drawn in the top plane and its prism from
// z = −2 to z = 0, and the ball of radius 3 about (0, 0, 1) joined to it, standing up out of the
// plate's top; the concave edge where they meet beveled: the frustum from radius 4 at z = 0 to
// radius 3 at z = 1, a turn of its half-section drawn in the front plane (x, z) about the z axis,
// joined too. Then the bevel, the frustum's cone, deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the bevel runs the faces round it on along their own surfaces until
// they meet, so what is left is bounded by the surfaces of every face of the solid but the
// bevel, read from its own faces: on the material's side of the plate's bottom and rim, and of
// the plate's top or the ball.
unit mm
use std
use std.sets
use std.solids
use std.solids (union)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 8)
  radius(8) rim
}
c := point
fix((0, 0, 1)) c

plate := solid(std.solids.Prism(face(rim), from: -2mm, to: 0mm))
globe := std.sets.Ball(c, r: 3mm)
ball := solid({ p | p in globe })
ball union plate

in std.front {
  f0 := point
  f1 := point hint((4, 0))
  f2 := point hint((3, 1))
  f3 := point hint((0, 1))
  horizontal (foot := line(f0, f1)) -> (bevel := line(f1, f2)) ->
  horizontal (crown := line(f2, f3)) -> vertical (core := line(f3, f0)) -> close
  fix((0, 0)) f0
  f0 distance(4) f1
  f0 distance(1) f3
  f3 distance(3) f2
}
frustum := solid(std.solids.Revolution(face(foot, bevel, crown, core), about: std.z))
frustum union plate

part := solid({ p |
  p in plate.far.side
  p in plate.rim.side
  p in plate.near.side or p in plate.ball.globe.side
})
