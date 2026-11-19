// A post on the z axis capped by a ball, made as one solid of two turns about the z axis of
// sections drawn in the front plane (x, z): the cylinder of radius 4 from z = 0 to z = 6 (its wall
// swept from `wall`), and the part above z = 6 of the ball of radius 5 about (0, 0, 3) (its dome
// from `dome`), whose turn starts 1.5 radians round, so the rim where the two meet is two edges.
// The rim rounded to radius 1, its round two faces. Then both deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face of
// the rounded post but the round's (its base, its wall and its dome), read from its own faces.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds (round)

in std.front {
  a := point
  b := point hint((4, 0))
  c := point hint((4, 6))
  m := point hint((0, 6))
  fix((0, 0)) a
  a distance(4) b
  b distance(6) c
  horizontal (base := line(a, b)) -> vertical (wall := line(b, c)) ->
  horizontal (seat := line(c, m)) -> vertical (spine := line(m, a)) -> close

  e := point hint((0, 8))
  o := point hint((0, 3))
  fix((0, 3)) o
  (dome := arc(center: o, start: c, end: e)) -> vertical (axis_cap := line(e, m)) ->
  horizontal (floor := line(m, c))
}

post := solid(std.solids.Revolution(face(base, wall, seat, spine), about: std.z))
cap := solid(std.solids.Revolution(face(dome, axis_cap, floor), about: std.z, from: 1.5rad,
  to: 1.5rad + 360deg))
cap union post
post.wall round(r: 1mm) post.cap.dome

part := solid({ p | p in post.base.side; p in post.wall.side; p in post.cap.dome.side })
