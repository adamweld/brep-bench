// A puck: a cylinder of radius 6 on the z axis from z = 0 to z = 10, both its rims rounded to
// radius 2. Hollowed to walls 3 thick, more than the rounds' radius, with no opening: the shell
// keeps every point within 3 of everything outside the puck, so the rounds moved in turn inside
// out and drop, and the closed cavity is a sharp cylinder of radius 3 from z = 3 to z = 7. The
// shell reads the puck as its rounds leave it, a solid of the rounded puck.
unit mm
use std
use std.solids
use std.rounds (round, shell)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 6)
  radius(6) rim
}

puck := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
round(r: 2mm) puck                  // both rims, every edge among its faces

part := solid(puck)                 // the puck as its rounds leave it
shell(t: 3mm) part
