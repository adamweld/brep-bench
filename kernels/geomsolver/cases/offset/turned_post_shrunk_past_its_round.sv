// A turned post with its shoulder rounded, shrunk by 1.5: its half-section drawn in the front plane
// (x, z), out to radius 5 from z = 0 to z = 6 and then in along the cone to radius 2 at z = 10,
// turned a whole turn about the z axis; the convex edge where the cylinder meets the cone rounded
// to radius 1, and the points of it at least 1.5 from its outside, `std.rounds.Grown`'s body for a
// negative reach.
unit mm
use std
use std.solids
use std.rounds
use std.rounds (round)

in std.front {
  a := point
  b := point hint((5, 0))
  c := point hint((5, 6))
  d := point hint((2, 10))
  e := point hint((0, 10))
  horizontal (base := line(a, b)) -> vertical (wall := line(b, c)) -> (cone := line(c, d)) ->
  horizontal (top := line(d, e)) -> vertical (axis_run := line(e, a)) -> close
  fix((0, 0)) a
  a distance(5) b
  b distance(6) c
  a distance(10) e
  e distance(2) d
  half := face(base, wall, cone, top, axis_run)
}

post := solid(std.solids.Revolution(half, about: std.z))
post.wall round(r: 1mm) post.cone
shrunk := std.rounds.Grown(post, d: -1.5mm)
part := solid(shrunk.body)
