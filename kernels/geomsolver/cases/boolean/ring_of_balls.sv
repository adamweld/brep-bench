// Twelve balls of radius 3 on a ring: one centre drawn in the top plane 10 from the origin, on
// the x axis, and a ring of twelve turns about the origin making the others; each copy's ball a
// solid. The part starts empty and each ball is joined to it in turn.
unit mm
use std
use std.solids (union)
use std.sets
use std.place (horizontal)

in std.top {
  o := point
  fix((0, 0)) o
  ring 12 about o {
    c := point hint((10, 0))
    o distance(10mm) c
    ball := solid(std.sets.Ball(c, r: 3mm))
  }
  o horizontal c[0]
}

part := solid()
repeat 12 as i {
  ball[i] union part
}
