// A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20; a ball of radius 1 about
// (5, 0, -2), carried by a screw about the z axis that turns right-handed (from +x toward +y) and
// rises 4 a turn, for six whole turns, and everything it passes through cut from the rod.
unit mm
use std
use std.sets
use std.solids
use std.solids (cut)
use std.motion (screws)

in std.top {
  centreline := point
  fix((0, 0)) centreline
  skin := circle(center: centreline) hint(r: 5)
  radius(5) skin
}
part := solid(std.solids.Prism(face(skin), from: 0mm, to: 20mm))

o := point
fix((5, 0, -2)) o
ball := solid(std.sets.Ball(o, r: 1mm))

climb := frame
climb screws(4mm) std.z
groove := solid({ p | some t in (0deg, 6 * 360deg): p in ball at(under: climb) t })
groove cut part
