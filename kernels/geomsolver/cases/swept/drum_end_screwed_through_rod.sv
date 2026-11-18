// A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20; an end mill, the cylinder
// of radius 1.2 about the x axis from x = 4 to x = 12 (its end disc drawn in the side plane and
// carried along x), carried by a screw about the z axis that turns right-handed (from +x toward
// +y) and rises 6 a turn, from a turn of -90 degrees to one of 1350, and everything it passes
// through cut from the rod.
unit mm
use std
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

in std.side {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 1.2)
  radius(1.2) rim
}
mill := solid(std.solids.Prism(face(rim), from: 4mm, to: 12mm))

climb := frame
climb screws(6mm) std.z
groove := solid({ p | some t in (-90deg, 1350deg): p in mill at(under: climb) t })
groove cut part
