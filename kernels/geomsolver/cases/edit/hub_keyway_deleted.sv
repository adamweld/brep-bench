// A hub: the circle of radius 10 about the origin drawn in the top plane, its bore the circle of
// radius 4 about the same centre a hole in it, and the prism of that face from z = 0 to z = 20.
// A keyway is cut along the bore's whole length: the rectangle from x = −1 to x = 1 and from
// y = −5 to y = 0, drawn in the top plane, and the prism through it cut away. Then the keyway's
// three faces deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the keyway runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face of
// the keyed hub but the keyway's (its two ends, its outside and its bore), read from its own
// faces.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 10)
  radius(10) rim
  bore := circle(center: o) hint(r: 4)
  radius(4) bore

  k0 := point hint((-1, -5))
  k1 := point hint((1, -5))
  k2 := point hint((1, 0))
  k3 := point hint((-1, 0))
  key := horizontal line(k0, k1) -> vertical line(k1, k2) ->
         horizontal line(k2, k3) -> vertical line(k3, k0) -> close
  fix((-1, -5)) k0
  fix((1, 0)) k2
  keyway := solid(std.solids.Through(key))
  keyway cut hub
}

hub := solid(std.solids.Prism(face(rim, holes: bore), from: 0mm, to: 20mm))

part := solid({ p |
  p in hub.near.side
  p in hub.far.side
  p in hub.rim.side
  p in hub.bore.side
})
