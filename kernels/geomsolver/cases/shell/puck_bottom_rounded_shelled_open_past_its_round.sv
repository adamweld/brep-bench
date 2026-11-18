// A puck: a cylinder of radius 6 on the z axis from z = 0 to z = 10, its bottom rim, where the far
// cap meets the side, rounded to radius 2. Shelled to walls 3 thick, more than the round's radius,
// and open through its top: the shell keeps every point within 3 of a face kept, so the round
// moved in turns inside out and drops, and the cavity is a cylinder of radius 3 with a sharp floor.
// The shell reads the puck as its round leaves it, a solid of the rounded puck.
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
puck.far round(r: 2mm) puck

part := solid(puck)                 // the puck as its round leaves it
part.puck.near shell(t: 3mm) part
