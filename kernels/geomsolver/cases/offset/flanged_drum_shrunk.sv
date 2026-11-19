// A flanged drum shrunk by 1: the drum's half-section drawn in the front plane (x, z), out from the
// axis to radius 8 from z = 0 to z = 6 with the flange standing out to radius 11 from z = 2.2 to
// z = 3.8, turned a whole turn about the z axis; and the points of it at least 1 from its outside,
// `std.rounds.Grown`'s body for a negative reach.
unit mm
use std
use std.solids
use std.rounds

in std.front {
  a := point
  b := point hint((8, 0))
  c := point hint((8, 2.2))
  d := point hint((11, 2.2))
  e := point hint((11, 3.8))
  f := point hint((8, 3.8))
  g := point hint((8, 6))
  h := point hint((0, 6))
  half := horizontal line(a, b) -> vertical line(b, c) -> horizontal line(c, d) ->
          vertical line(d, e) -> horizontal line(e, f) -> vertical line(f, g) ->
          horizontal line(g, h) -> vertical line(h, a) -> close
  fix((0, 0)) a
  a distance(8) b
  a distance(6) h
  b distance(2.2) c
  c distance(3) d
  d distance(1.6) e
  e distance(3) f
}

drum := solid(std.solids.Revolution(half, about: std.z))
shrunk := std.rounds.Grown(drum, d: -1mm)
part := solid(shrunk.body)
