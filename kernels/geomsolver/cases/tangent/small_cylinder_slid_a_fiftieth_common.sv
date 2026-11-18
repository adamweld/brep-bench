// A small cylinder of radius 0.00075 and height 0.0015 on the z axis, the prism from z = 0 over a
// circle about the origin of the top plane (x, y), and a copy of it slid 0.000015 along x: the
// prism between the same planes over the circle of the same radius about a centre 0.000015 out
// along x. Their common part.
unit mm
use std
use std.solids
use std.solids (common)
use std.place (horizontal)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 0.00075)
  radius(0.00075) rim

  c := point hint((0.000015, 0))
  o distance(0.000015mm, along: u) c
  o horizontal c
  slid := circle(center: c) hint(r: 0.00075)
  slid equal rim
}

copy := solid(std.solids.Prism(face(slid), from: 0mm, to: 0.0015mm))
part := solid(std.solids.Prism(face(rim), from: 0mm, to: 0.0015mm))
copy common part
