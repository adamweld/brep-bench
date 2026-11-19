// A shaft, the circle of radius 5 about the origin drawn in the top plane and its prism from
// z = 0 to z = 20, with a flat milled along its whole length: everything with y < −4 cut away,
// by the prism through a rectangle drawn in the top plane from y = −4 out past the shaft. Then
// the flat deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit is
// said as one: deleting the flat runs the faces round it on along their own surfaces until they
// meet, so what is left is the points on the material's side of the surface of every face of the
// milled shaft but the flat (its two ends and its cylinder), read from its own faces.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim
}
shaft := solid(std.solids.Prism(face(rim), from: 0mm, to: 20mm))

in std.top {
  slab0 := point hint((-6, -6))
  slab1 := point hint((6, -6))
  slab2 := point hint((6, -4))
  slab3 := point hint((-6, -4))
  slab := horizontal line(slab0, slab1) -> vertical line(slab1, slab2) ->
         horizontal line(slab2, slab3) -> vertical line(slab3, slab0) -> close
  fix((-6, -6)) slab0
  fix((6, -4)) slab2
}
mill := solid(std.solids.Through(slab))
mill cut shaft
part := solid({ p | p in shaft.near.side; p in shaft.far.side; p in shaft.rim.side })
