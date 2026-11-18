// A plate, its 40 × 30 footprint drawn in the top plane (x, y) from the origin and the prism over
// it to z = 8, drilled right through along z by a hole about (20, 15): the circle's prism through
// the plate cut from it. The hole was drawn of radius 5; then its wall is offset by 2 into the
// plate.
//
// Solvent moves no face of a built solid: a solid is the points of a set, and nothing about it is
// solved for. The hole's wall is its circle swept through the plate, so the edit is the circle's
// radius: drawn 5, offset by `offset` away from its axis. The top and the bottom are the
// footprint's less the hole's, so they are trimmed back to meet it.
unit mm
use std
use std.solids
use std.solids (cut)

drawn := 5mm    // the hole as it was drawn
offset := 2mm   // the edit: its wall offset into the plate

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

  o := point hint((20, 15))
  a distance(20mm, along: u) o
  a distance(15mm, along: v) o
  hole := circle(center: o) hint(r: 7)
  radius(drawn + offset) hole
  drill := solid(std.solids.Through(face(hole)))
  drill cut part
}

part := solid(std.solids.Prism(outline, from: 0mm, to: 8mm))
