// A curved block, the ring sector between radii 8 and 12 about the z axis, z within 1, polar angle
// from 0 to 60 degrees: its section in the front plane (x, z) turned 60 degrees about the z axis.
// A tapered cutter, the frustum about the upright line through (10, 0) from radius 0.4 at z = -0.6
// to radius 1 at z = 1.4, a turn of its half-section in the front plane; carried by a screw about
// the z axis that turns right-handed (from +x toward +y) and rises 2 a turn, from a turn of -140
// degrees to one of +140, and everything it passes through cut from the block.
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

in std.front {
  foot := point
  toe := point hint((10.4, -0.6))
  brim := point hint((11, 1.4))
  crown := point hint((10, 1.4))
  fix((10, -0.6)) foot
  fix((10, 1.4)) crown
  foot distance(0.4) toe
  crown distance(1) brim
  half := horizontal line(foot, toe) -> line(toe, brim) -> horizontal line(brim, crown) ->
          vertical (spindle := line(crown, foot)) -> close
}
cutter := solid(std.solids.Revolution(half, about: spindle))

climb := frame
climb screws(2mm) std.z
groove := solid({ p | some t in (-140deg, 140deg): p in cutter at(under: climb) t })
groove cut part
