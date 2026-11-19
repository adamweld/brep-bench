// The solid a thick disc fills as it is turned partway about an upright line through its rim: the
// cylinder of radius 2 about the line through (2, 0, 0) along y, from y = 0 to y = 0.5 (its face
// drawn in the front plane and carried along y), turned rigidly 90 degrees about the z axis,
// right-handed (from +x toward +y).
unit mm
use std
use std.solids
use std.motion (turns)

// the front plane's normal is -y, so the disc stands from 0 to 0.5 behind it
in std.front {
  o := point
  fix((2, 0)) o
  rim := circle(center: o) hint(r: 2)
  radius(2) rim
}
coin := solid(std.solids.Prism(face(rim), from: -0.5mm, to: 0mm))

swing := frame
swing turns(1) std.z
part := solid({ p | some t in (0deg, 90deg): p in coin at(under: swing) t })
