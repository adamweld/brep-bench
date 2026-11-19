// A post on the z axis capped by a ball, a turn of its half-section drawn in the front plane (x, z)
// about the z axis, made as one solid: the cylinder of radius 4 from z = 0 to z = 6 (swept from
// `wall`) and the part above z = 6 of the ball of radius 5 about (0, 0, 3), whose arc runs from
// the wall's top to the axis (from `dome`). The rim where the wall meets the dome rounded to
// radius 1. Then the round deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face of
// the rounded post but the round's (its base, its wall and its dome), read from its own faces.
unit mm
use std
use std.solids
use std.rounds (round)

in std.front {
  a := point
  b := point hint((4, 0))
  c := point hint((4, 6))
  e := point hint((0, 8))
  o := point hint((0, 3))
  fix((0, 0)) a
  fix((0, 3)) o
  a distance(4) b
  b distance(6) c
  horizontal (base := line(a, b)) -> vertical (wall := line(b, c)) ->
  (dome := arc(center: o, start: c, end: e)) -> vertical (core := line(e, a)) -> close
}

post := solid(std.solids.Revolution(face(base, wall, dome, core), about: std.z))
post.wall round(r: 1mm) post.dome

part := solid({ p | p in post.base.side; p in post.wall.side; p in post.dome.side })
