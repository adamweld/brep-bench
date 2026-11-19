// A grooved drum grown by 1: the drum's half-section drawn in the front plane (x, z), out from the
// axis to radius 10 from z = 0 to z = 10 with the groove's notch cut 3 deep (to radius 7) from
// z = 4.2 to z = 5.8, turned a whole turn about the z axis; and the points within 1 of it,
// `std.rounds.Grown`'s body.
unit mm
use std
use std.solids
use std.rounds

in std.front {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 4.2))
  d := point hint((7, 4.2))
  e := point hint((7, 5.8))
  f := point hint((10, 5.8))
  g := point hint((10, 10))
  h := point hint((0, 10))
  half := horizontal line(a, b) -> vertical line(b, c) -> horizontal line(c, d) ->
          vertical line(d, e) -> horizontal line(e, f) -> vertical line(f, g) ->
          horizontal line(g, h) -> vertical line(h, a) -> close
  fix((0, 0)) a
  a distance(10) b
  a distance(10) h
  b distance(4.2) c
  c distance(3) d
  d distance(1.6) e
  e distance(3) f
}

drum := solid(std.solids.Revolution(half, about: std.z))
grown := std.rounds.Grown(drum, d: 1mm)
part := solid(grown.body)
