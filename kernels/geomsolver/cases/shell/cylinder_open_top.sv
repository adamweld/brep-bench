// A cylinder of radius 5 on the z axis from z = 0 to z = 10, shelled to walls 1 thick and open
// through its top, `part.near`: a cup.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5) rim
}

part := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))
part.near shell(t: 1mm) part
