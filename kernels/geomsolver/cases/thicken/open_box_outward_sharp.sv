// An open box's sheet — the floor and four walls of the cube from the origin to (10, 10, 10),
// with no top — thickened by 1 out of the cube, its edges kept sharp. Solvent's thickening of a
// sheet, a developable panel's band, takes one ruled panel between two rails, so a sheet of five
// faces meeting at creases and corners has none; drawn instead as what the thickening is: each face
// moved 1 out along its normal and extended until it meets the others — the walls' faces bound the
// square 1 outside the cube's, the floor's lies at z = −1 — the prism over that square from z = −1
// up to the cube's top, less the cube.
unit mm
use std
use std.solids
use std.solids (cut)
use std.place (offset)

a_side := 10mm
t := 1mm

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 10))
  d := point hint((0, 10))
  square := horizontal line(a, b) -> vertical line(b, c) ->
            horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(a_side) b
  b distance(a_side) c
  // the walls moved t out: the square whose corners stand t out from the cube's both ways
  e := point hint((-1, -1))
  f := point hint((11, -1))
  g := point hint((11, 11))
  h := point hint((-1, 11))
  outer := horizontal line(e, f) -> vertical line(f, g) ->
           horizontal line(g, h) -> vertical line(h, e) -> close
  e offset(dx: -t, dy: -t) a
  g offset(dx: t, dy: t) c
}
cube := solid(std.solids.Prism(square, from: 0mm, to: a_side))
part := solid(std.solids.Prism(outer, from: -t, to: a_side))
cube cut part
