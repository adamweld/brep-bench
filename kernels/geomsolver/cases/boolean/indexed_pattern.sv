// A disk of radius 20 from z = −5 to z = 0, and six holes of radius 2 indexed round it: one hole's
// centre drawn in the top plane 12 from the axis on the x axis, a ring of six turns about the axis
// making the others, each copy's cutter through the disk along z; each is cut in its own turn.
unit mm
use std
use std.solids
use std.solids (cut)
use std.place (horizontal)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 20)
  radius(20mm) rim
  ring 6 about o {
    c := point hint((12, 0))
    o distance(12mm) c
    hole := circle(center: c) hint(r: 2)
    radius(2mm) hole
    drill := solid(std.solids.Through(face(hole)))
  }
  o horizontal c[0]
}

part := solid(std.solids.Prism(face(rim), from: -5mm, to: 0mm))
repeat 6 as i {
  drill[i] cut part
}
