// A curved block, the ring sector between radii 8 and 12 about the z axis, z within 1, polar angle
// from 0 to 60 degrees: its section in the front plane (x, z) turned 60 degrees about the z axis.
// A bull-nose mill about the upright line through (10, 0): radius 1 from z = -0.5 to 0.5, its
// bottom corner rounded to 0.3, a turn of its half-section in the front plane. It is turned
// rigidly about the z axis from -140 to +30 degrees, and everything it passes through cut from the
// block.
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
  e0 := point
  q := point hint((11, -0.5))
  e2 := point hint((11, 0.5))
  e3 := point hint((10, 0.5))
  fix((10, -0.5)) e0
  e0 distance(1) q
  e0 distance(1) e3
  // the side meets the flat bottom at q, the corner rounded to 0.3
  half := horizontal line(e0, q) -> round(r: 0.3mm) vertical line(q, e2) ->
          horizontal line(e2, e3) -> vertical (spindle := line(e3, e0)) -> close
}
mill := solid(std.solids.Revolution(half, about: spindle))

swing := frame
swing turns(1) std.z
notch := solid({ p | some t in (-140deg, 30deg): p in mill at(under: swing) t })
notch cut part
