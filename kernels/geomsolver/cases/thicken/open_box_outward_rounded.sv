// An open box's sheet — the floor and four walls of the cube from the origin to (10, 10, 10),
// with no top — thickened by 1 out of the cube, its edges rounded. Solvent's thickening of a
// sheet, a developable panel's band, takes one ruled panel between two rails, so a sheet of five
// faces meeting at creases and corners has none; but the sheet bounds the cube on all sides but its
// top, and thickened outward with every face moved 1 along its normal and the gaps at its edges and
// corners closed by the quarter cylinders and eighth balls about them, it is the cube grown by 1 —
// `std.rounds.Grown`, its edges and corners rounded by 1 — no higher than the cube's top, less the
// cube.
unit mm
use std
use std.solids
use std.solids (cut, common)
use std.rounds
use std.shapes

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
  // a square wider than the grown cube, to keep what lies under the cube's top
  wide := std.shapes.CenteredRectangle(std.top.origin, w: 40mm, h: 40mm)
}
cube := solid(std.solids.Prism(square, from: 0mm, to: 10mm))
grown := std.rounds.Grown(cube, d: 1mm)
part := solid(grown.body)
below_top := solid(std.solids.Prism(wide.loop, from: -20mm, to: 10mm))  // under the top's plane
below_top common part
cube cut part
