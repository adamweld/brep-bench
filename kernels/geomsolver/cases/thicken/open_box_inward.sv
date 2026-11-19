// An open box's sheet — the floor and four walls of the cube from the origin to (10, 10, 10),
// with no top — thickened by 1 into the cube. Solvent's thickening of a sheet, a developable panel's
// band, takes one ruled panel between two rails, so a sheet of five faces meeting at creases and
// corners has none; but the sheet bounds the cube on all sides but its top, and thickened inward
// it is the cube shelled to walls 1 thick, open through its top: each wall its face moved in by 1,
// meeting sharp at the convex edges.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 10))
  d := point hint((0, 10))
  square := horizontal line(a, b) -> vertical line(b, c) ->
            horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(10) c
}
part := solid(std.solids.Prism(square, from: 0mm, to: 10mm))
part.near shell(t: 1mm) part                  // `near` the top, at z = 10: left open
