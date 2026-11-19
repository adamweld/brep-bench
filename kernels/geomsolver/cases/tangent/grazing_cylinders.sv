// Two cylinders of radius 5 and length 30 crossing square and barely overlapping: the first on the x
// axis, its circle drawn about the origin of the side plane (y, z) and the prism over it from
// x = -15 to x = 15; the second parallel to the y axis on the line x = 0, z = 9.99, its circle drawn
// in the front plane (x, z) 9.99 up from the origin and the prism over it from y = -15 to y = 15.
// Their common part, a cushion 0.01 thick.
unit mm
use std
use std.solids
use std.solids (common)
use std.place (vertical)

in std.side {
  o := point
  fix((0, 0)) o
  first := circle(center: o) hint(r: 5)
  radius(5) first
}

in std.front {
  q := point
  fix((0, 0)) q
  c := point hint((0, 10))
  q distance(9.99mm, along: v) c
  q vertical c
  second := circle(center: c) hint(r: 5)
  second equal first
}

across := solid(std.solids.Prism(face(second), from: -15mm, to: 15mm))
part := solid(std.solids.Prism(face(first), from: -15mm, to: 15mm))
across common part
