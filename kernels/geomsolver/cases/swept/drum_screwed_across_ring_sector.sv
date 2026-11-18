// A curved block, the ring sector between radii 8 and 12 about the z axis, z within 1, polar angle
// from 0 to 60 degrees: its section in the front plane (x, z) turned 60 degrees about the z axis.
// A drum, the cylinder of radius 1 about the upright line through (10, 0) from z = -0.5 to 0.5,
// carried by a screw about the z axis that turns right-handed (from +x toward +y) and rises 0.5 a
// turn, from a turn of -140 degrees to one of +140, and everything it passes through cut from the
// block.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (screws)

in std.front {
  a := point
  b := point hint((12, -1))
  c := point hint((12, 1))
  d := point hint((8, 1))
  meridian := horizontal line(a, b) -> vertical line(b, c) ->
              horizontal line(c, d) -> vertical line(d, a) -> close
  fix((8, -1)) a
  a distance(4) b
  b distance(2) c
}
part := solid(std.solids.Revolution(meridian, about: std.z, to: 60deg))

in std.top {
  o := point
  fix((10, 0)) o
  rim := circle(center: o) hint(r: 1)
  radius(1) rim
}
drum := solid(std.solids.Prism(face(rim), from: -0.5mm, to: 0.5mm))

climb := frame
climb screws(0.5mm) std.z
groove := solid({ p | some t in (-140deg, 140deg): p in drum at(under: climb) t })
groove cut part
