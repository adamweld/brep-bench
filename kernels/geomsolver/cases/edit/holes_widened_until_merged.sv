// A plate, its 40 × 30 footprint drawn in the top plane (x, y) from the origin and the prism over
// it to z = 8, drilled right through along z by two holes about (15, 15) and (25, 15), each the
// circle's prism through the plate cut from it. The holes were drawn of radius 3; then both walls
// are offset by 3 into the plate, until they overlap.
//
// Solvent moves no face of a built solid: a solid is the points of a set, and nothing about it is
// solved for. Each wall is its circle swept through the plate, so the edit is the circles' radius:
// drawn 3, offset by `offset` away from their axes. The plate is the footprint's prism less both
// holes' points, so the walls meet where the holes overlap, each inside the other is gone, and
// the top and the bottom are trimmed to the opening the two make.
unit mm
use std
use std.solids
use std.solids (cut)

drawn := 3mm    // the holes as they were drawn
offset := 3mm   // the edit: their walls offset into the plate

in std.top {
  a := point
  b := point hint((40, 0))
  c := point hint((40, 30))
  d := point hint((0, 30))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(40) b
  b distance(30) c

  repeat 2 as i {
    o := point hint((15 + 10 * i, 15))
    a distance(15mm + 10mm * i, along: u) o
    a distance(15mm, along: v) o
    hole := circle(center: o) hint(r: 6)
    radius(drawn + offset) hole
    drill := solid(std.solids.Through(face(hole)))
    drill cut part
  }
}

part := solid(std.solids.Prism(outline, from: 0mm, to: 8mm))
