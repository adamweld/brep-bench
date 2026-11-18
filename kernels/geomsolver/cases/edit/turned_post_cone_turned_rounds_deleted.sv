// A turned post on the z axis, made as one solid of two turns about the z axis of sections drawn
// in the front plane (x, z): the cylinder of radius 5 from z = 0 to z = 6 (its wall swept from
// `wall`), and the cone frustum from radius 5 at z = 6 to radius 2 at z = 10 (its cone from
// `cone`), whose turn starts a radian round, so the shoulder where the two meet is two edges.
// The shoulder rounded to radius 1, its round two faces. Then both deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face of
// the rounded post but the round's (its base, its wall, its cone and its top), read from its own
// faces.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds (round)

in std.front {
  a := point
  b := point hint((5, 0))
  c := point hint((5, 6))
  m := point hint((0, 6))
  horizontal (base := line(a, b)) -> vertical (wall := line(b, c)) ->
  horizontal (seat := line(c, m)) -> vertical (spine := line(m, a)) -> close
  fix((0, 0)) a
  a distance(5) b
  b distance(6) c

  d := point hint((2, 10))
  e := point hint((0, 10))
  (cone := line(c, d)) -> horizontal (top := line(d, e)) -> vertical (axis_cap := line(e, m)) ->
  horizontal (floor := line(m, c))
  a distance(10) e
  e distance(2) d
}

post := solid(std.solids.Revolution(face(base, wall, seat, spine), about: std.z))
cap := solid(std.solids.Revolution(face(cone, top, axis_cap, floor), about: std.z, from: 1rad,
  to: 1rad + 360deg))
cap union post
post.wall round(r: 1mm) post.cap.cone

part := solid({ p |
  p in post.base.side
  p in post.wall.side
  p in post.cap.cone.side
  p in post.cap.top.side
})
