// A square sheet with two holes, thickened by 1 to each side. The sheet is a face, the square from
// (−10, −10) to (10, 10) in the top plane with two circles of radius 3 about (±5, 0) as its
// holes, and the thickened sheet is that face slid along its normal from −1 to 1: its prism,
// holed through by the walls its circles sweep. (Solvent's thickening of a sheet, a developable
// panel's band, takes a panel between two rails, so it has no flat sheet with holes and no band to
// both sides.)
unit mm
use std
use std.shapes
use std.solids

in std.top {
  o := point
  fix((0, 0)) o
  sq := std.shapes.CenteredRectangle(o, w: 20mm, h: 20mm)
  cl := point hint((-5, 0))
  cr := point hint((5, 0))
  o distance(-5mm, along: u) cl
  o level(v) cl
  o distance(5mm, along: u) cr
  o level(v) cr
  hole_l := circle(center: cl) hint(r: 3)
  hole_r := circle(center: cr) hint(r: 3)
  radius(3mm) hole_l
  radius(3mm) hole_r
  sheet := face(sq.loop, holes: hole_l, hole_r)
}
part := solid(std.solids.Prism(sheet, from: -1mm, to: 1mm))
