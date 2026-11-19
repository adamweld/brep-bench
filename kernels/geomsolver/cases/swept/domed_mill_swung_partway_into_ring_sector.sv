// A curved block, the ring sector between radii 8 and 12 about the z axis, z within 1, polar angle
// from 0 to 60 degrees: its section in the front plane (x, z) turned 60 degrees about the z axis.
// A domed mill about the upright line through (10, 0): a cylinder of radius 1 up to z = 3, ended
// below by a cap of the sphere of radius 2 about (10, 0, 1.2), its lowest point (10, 0, -0.8); a
// turn of its half-section in the front plane. It is turned rigidly about the z axis from -140 to
// +30 degrees, and everything it passes through cut from the block.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (turns)

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
  tip := point
  k := point hint((10, 1.2))
  lip := point hint((11, -0.53))
  e2 := point hint((11, 3))
  e3 := point hint((10, 3))
  fix((10, -0.8)) tip
  fix((10, 3)) e3
  k coincident spindle
  tip distance(1, along: u) lip
  // the cap, from its lowest point round to the sharp rim where it meets the side
  half := (cap := arc(center: k, start: tip, end: lip)) -> vertical line(lip, e2) ->
          horizontal line(e2, e3) -> vertical (spindle := line(e3, tip)) -> close
  radius(2) cap
}
mill := solid(std.solids.Revolution(half, about: spindle))

swing := frame
swing turns(1) std.z
notch := solid({ p | some t in (-140deg, 30deg): p in mill at(under: swing) t })
notch cut part
