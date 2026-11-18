// A block, its 40 × 30 footprint drawn in the top plane (x, y) from the origin and the prism over
// it, drilled right through along z by a hole of radius 5 about (20, 15), the circle's prism
// through the block cut from it. Then its top moved up by 6.
//
// Solvent moves no face of a built solid: a solid is the points of a set, and nothing about it is
// solved for. Its top is where the prism's greater ordinate puts it, so the edit is that
// dimension's: the top drawn at z = 10, moved up by `lift`. The sides and the hole's wall are the
// footprint's and the hole's swept between the ordinates, so they run on up to meet it.
unit mm
use std
use std.solids
use std.solids (cut)

top := 10mm    // where the top was
lift := 6mm    // the edit: the top moved up, along +z

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
  hole := circle(center: o) hint(r: 5)
  radius(5) hole
  drill := solid(std.solids.Through(face(hole)))
  drill cut part
}

part := solid(std.solids.Prism(outline, from: 0mm, to: top + lift))
