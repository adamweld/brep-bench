// A box from the origin to (20, 10, 6), the four edges round its top face chamfered 1 across the
// top and 2 down each side. The library's `chamfer(d: …)` cuts one distance on both faces; its
// flat, `std.rounds.Flat(a, b, la: …, lb: …)`, takes a leg on each, so a word of two legs is a few
// lines here: for each face `a` of A and `b` of B that meet, the flat `across` on `a` and `down` on
// `b`, cut from the body. Neighbouring flats mitre on the line from the box's corner at z = 4 to
// the top's corner.
unit mm
use std
use std.solids
use std.solids (cut)
use std.rounds

A bevel(across, down) B := {
  repeat a in A {
    repeat b in B {
      when b != a and b meets a {
        flat := std.rounds.Flat(a, b, la: across, lb: down)
        taken := solid(flat)
        taken cut A.body
      }
    }
  }
}

in std.top {
  a := point
  b := point hint((20, 0))
  c := point hint((20, 10))
  d := point hint((0, 10))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(20) b
  b distance(10) c
}

part := solid(std.solids.Prism(footprint, from: 0mm, to: 6mm))
part.near bevel(across: 1mm, down: 2mm) part
