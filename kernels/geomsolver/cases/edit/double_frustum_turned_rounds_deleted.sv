// Two cone frustums on the z axis made as one solid of two turns about the z axis of sections
// drawn in the front plane (x, z): the lower cone (swept from `lower`) widening from radius 3 at
// z = 0 to radius 5 at z = 4, and the upper (from `upper`) narrowing from there to radius 2 at
// z = 8, whose turn starts two radians round, so the ridge where the two meet is two edges. The
// ridge rounded to radius 1, its round two faces. Then both deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face of
// the rounded solid but the round's (its base, its two cones and its top), read from its own
// faces.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds (round)

in std.front {
  a := point
  b := point hint((3, 0))
  c := point hint((5, 4))
  m := point hint((0, 4))
  horizontal (base := line(a, b)) -> (lower := line(b, c)) ->
  horizontal (waist := line(c, m)) -> vertical (spine := line(m, a)) -> close
  fix((0, 0)) a
  a distance(3) b
  fix((5, 4)) c

  d := point hint((2, 8))
  e := point hint((0, 8))
  (upper := line(c, d)) -> horizontal (top := line(d, e)) -> vertical (axis_cap := line(e, m)) ->
  horizontal (floor := line(m, c))
  a distance(8) e
  e distance(2) d
}

pair := solid(std.solids.Revolution(face(base, lower, waist, spine), about: std.z))
cap := solid(std.solids.Revolution(face(upper, top, axis_cap, floor), about: std.z, from: 2rad,
  to: 2rad + 360deg))
cap union pair
pair.lower round(r: 1mm) pair.cap.upper

part := solid({ p |
  p in pair.base.side
  p in pair.lower.side
  p in pair.cap.upper.side
  p in pair.cap.top.side
})
