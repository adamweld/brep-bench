// The solid a rod fills as it is spun partway about an upright line through it. The rod is a
// cylinder of radius 0.5 about the x axis from x = -3 to 5: its circle drawn in the side plane
// (y, z), whose normal is +x, and the prism over it from -3 to 5 along that normal. The part is
// the kernel's sweep of the rod turned right-handed about z through 120 degrees.
unit mm
use std
use std.solids
use std.motion (turns)

in std.side {
  private o := point
  fix((0, 0)) o
  private rim := circle(center: o) hint(r: 0.5)
  radius(0.5mm) rim
}
construction rod := solid(std.solids.Prism(face(rim), from: -3mm, to: 5mm))

spin := frame
spin turns(1) std.z
part := solid({ p | some t in (0deg, 120deg): p in rod at(under: spin) t })
