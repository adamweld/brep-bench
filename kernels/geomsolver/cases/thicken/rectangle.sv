// A rectangular sheet thickened by 2 to its +z side. The sheet is a face, the rectangle from the
// origin to (10, 8) in the top plane, and the thickened sheet is that face slid along its normal
// from 0 to 2: its prism. (Solvent's thickening of a sheet, a developable panel's band, refuses a
// flat panel between two parallel edges — "the rulings fold" — so a flat sheet is thickened as the
// prism of its face.)
unit mm
use std
use std.solids

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 8))
  d := point hint((0, 8))
  sheet := horizontal line(a, b) -> vertical line(b, c) ->
           horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(8) c
}
part := solid(std.solids.Prism(sheet, from: 0mm, to: 2mm))
