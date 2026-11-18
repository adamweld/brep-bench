// A cylinder of radius 10 along the x axis from x = −20 to 20, and a square pad 6 × 6 drawn in the
// plane z = 15 and extruded down to the cylinder, joined to it. Solvent has no extrusion up to a
// face (its prism's points below the square and not in the cylinder have no end, since they go on
// under it), so the pad's prism runs down 15, to the cylinder's axis, and what of it lies inside
// the cylinder is swallowed by the union.
unit mm
use std
use std.sets
use std.shapes
use std.solids
use std.solids (union)

west := point
fix((-20, 0, 0)) west
east := point
fix((20, 0, 0)) east
x_line := line(west, east)
rod := std.sets.Rod(x_line, r: 10mm)

// the pad's plane, 15 above the top plane
deck := plane
fix(origin == (0mm, 0mm, 15mm)) deck
fix(dir == (1, 0, 0)) deck.u
fix(dir == (0, 1, 0)) deck.v
in deck {
  sq := std.shapes.CenteredRectangle(deck.origin, w: 6mm, h: 6mm)
  pad_f := face(sq.loop)
}
pad := solid(std.solids.Prism(pad_f, from: -15mm, to: 0mm))
part := solid(rod)
pad union part
