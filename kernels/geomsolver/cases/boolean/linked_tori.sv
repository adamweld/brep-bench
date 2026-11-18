// Two rings linked through each other, each a tube's circle of radius 0.5 turned a whole turn: the
// first drawn in the front plane (x, z) 3 out along x and turned about the z axis; the second
// drawn in the top plane (x, y) at (6, 0) and turned about the line through (3, 0) parallel to y,
// so its middle circle, radius 3 about (3, 0, 0), lies in the plane y = 0. The part is the first
// with the second in union: two bodies that do not touch.
unit mm
use std
use std.solids
use std.solids (union)
use std.place (right_of)

in std.front {
  c1 := point hint((3, 0))
  c1 right_of(d: 3mm) std.origin
  tube1 := circle(center: c1) hint(r: 0.5)
  radius(0.5mm) tube1
}

in std.top {
  o := point
  fix((0, 0)) o
  hub := point hint((3, 0))
  hub right_of(d: 3mm) o
  up := point hint((3, 1))
  axis2 := vertical line(hub, up)
  hub distance(1mm) up
  c2 := point hint((6, 0))
  c2 right_of(d: 3mm) hub
  tube2 := circle(center: c2) hint(r: 0.5)
  radius(0.5mm) tube2
}

second := solid(std.solids.Revolution(face(tube2), about: axis2))
part := solid(std.solids.Revolution(face(tube1), about: std.z))
second union part
