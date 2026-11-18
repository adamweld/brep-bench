// A curved block, the ring sector between radii 8 and 12 about the z axis, z within 1, polar angle
// from 0 to 60 degrees: its section in the front plane (x, z) turned 60 degrees about the z axis.
// A half drum, the half with y <= 0 of the cylinder of radius 1 about the upright line through
// (10, 0) from z = -0.5 to 0.5: its footprint the half disc under the diameter from (9, 0) to
// (11, 0). It is turned rigidly about the z axis from -140 to +30 degrees, flat face first, and
// everything it passes through cut from the block.
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

in std.top {
  o := point
  fix((10, 0)) o
  w := point hint((9, 0))
  e := point hint((11, 0))
  w distance(1) o
  o distance(1) e
  // counter-clockwise from w round under the centre to e, and back along the diameter
  footprint := (round := arc(center: o, start: w, end: e)) -> horizontal (flat := line(e, w)) ->
               close
}
half := solid(std.solids.Prism(footprint, from: -0.5mm, to: 0.5mm))

swing := frame
swing turns(1) std.z
notch := solid({ p | some t in (-140deg, 30deg): p in half at(under: swing) t })
notch cut part
