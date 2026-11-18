// A block, its 40 × 30 footprint drawn in the top plane (x, y) from the origin and the prism over
// it, the edge where its top (`near`) meets its back y = 30 (swept from `north`) rounded to
// radius 3. Then the top moved up by 6, carrying the round.
//
// Solvent moves no face of a built solid: a solid is the points of a set, and nothing about it is
// solved for. The top is where the prism's greater ordinate puts it, so the edit is that
// dimension's: the top drawn at z = 10, moved up by `lift`. The round is said of the faces it
// joins, not of where they stand, so it stays a round of radius 3 between the moved top and the
// back, and the back and the two ends run on up to meet them.
unit mm
use std
use std.solids
use std.rounds (round)

top := 10mm    // where the top was
lift := 6mm    // the edit: the top moved up, along +z

in std.top {
  a := point
  b := point hint((40, 0))
  c := point hint((40, 30))
  d := point hint((0, 30))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
  horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  fix((0, 0)) a
  a distance(40) b
  b distance(30) c
}

part := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: top + lift))
part.near round(r: 3mm) part.north
