// Two spherical sectors sharing an apex, in common: the ball of radius 10 about the origin, within
// 45° of the z axis toward +z and within 45° of that axis turned 60° about the y axis, toward
// (sin 60°, 0, cos 60°) — the points of the ball inside both cones with their apex at the origin.
unit mm
use std
use std.sets

in std.front {
  o := point
  fix((0, 0)) o
  up := point hint((0, 1))
  first := line(o, up)
  vertical first
  length(1mm) first
  aslant := point hint((0.87, 0.5))
  second := line(o, aslant)
  length(1mm) second
  first angle(60deg, sense: cw) second
}

ball := std.sets.Ball(o, r: 10mm)
part := solid({ p |
  p in ball
  p in std.sets.Cone(first, half: 45deg)
  p in std.sets.Cone(second, half: 45deg)
})
