// A shaft, the circle of radius 5 about the origin drawn in the top plane and its prism from
// z = 0 to z = 20, with two opposite flats milled along its whole length: everything with
// |x| > 4 cut away, by the prisms through two rectangles drawn in the top plane from x = 4 and
// from x = −4 out past the shaft. Then both flats deleted together.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit is
// said as one: deleting the flats runs the faces round them on along their own surfaces until they
// meet, so what is left is the points on the material's side of the surface of every face of the
// milled shaft but the flats (its two ends and its cylinder, in two pieces), read from its own
// faces.
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
  east0 := point hint((4, -6))
  east1 := point hint((6, -6))
  east2 := point hint((6, 6))
  east3 := point hint((4, 6))
  east := horizontal line(east0, east1) -> vertical line(east1, east2) ->
         horizontal line(east2, east3) -> vertical line(east3, east0) -> close
  fix((4, -6)) east0
  fix((6, 6)) east2
}
in std.top {
  west0 := point hint((-6, -6))
  west1 := point hint((-4, -6))
  west2 := point hint((-4, 6))
  west3 := point hint((-6, 6))
  west := horizontal line(west0, west1) -> vertical line(west1, west2) ->
         horizontal line(west2, west3) -> vertical line(west3, west0) -> close
  fix((-6, -6)) west0
  fix((-4, 6)) west2
}
mill_east := solid(std.solids.Through(east))
mill_west := solid(std.solids.Through(west))
mill_east cut shaft
mill_west cut shaft
part := solid({ p | p in shaft.near.side; p in shaft.far.side; p in shaft.rim.side })
