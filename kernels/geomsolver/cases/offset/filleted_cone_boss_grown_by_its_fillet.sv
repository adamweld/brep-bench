// A filleted cone boss grown by 1: the plate the prism over a circle of radius 8 about the origin
// of the top plane, from z = −2 to z = 0; the boss the frustum from radius 4 at z = 0 to radius 1
// at z = 6, its half-section drawn in the front plane (x, z) and turned about the z axis, in union
// with it; the concave edge where the boss's cone meets the plate's top rounded to radius 1; and
// the points within 1 of all that, `std.rounds.Grown`'s body.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds
use std.rounds (round)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 8)
  radius(8) rim
}

in std.front {
  a := point
  b := point hint((4, 0))
  c := point hint((1, 6))
  d := point hint((0, 6))
  horizontal (foot := line(a, b)) -> (flank := line(b, c)) -> horizontal (top := line(c, d)) ->
  vertical (axis_run := line(d, a)) -> close
  fix((0, 0)) a
  a distance(4) b
  a distance(6) d
  d distance(1) c
  half := face(foot, flank, top, axis_run)
}

body := solid(std.solids.Prism(face(rim), from: -2mm, to: 0mm))
boss := solid(std.solids.Revolution(half, about: std.z))
boss union body
body.boss.flank round(r: 1mm) body.near
grown := std.rounds.Grown(body, d: 1mm)
part := solid(grown.body)
