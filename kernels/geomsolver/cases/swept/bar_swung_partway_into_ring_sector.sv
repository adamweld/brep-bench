// A curved block, the ring sector between radii 8 and 12 about the z axis, z within 1, polar angle
// from 0 to 60 degrees: its section in the front plane (x, z) turned 60 degrees about the z axis.
// A small bar, the box 10 <= x <= 11, |y| <= 0.5, |z| <= 0.6, turned rigidly about the z axis from
// -140 to +30 degrees, and everything it passes through cut from the block.
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
  e := point
  f := point hint((11, -0.5))
  g := point hint((11, 0.5))
  h := point hint((10, 0.5))
  footprint := horizontal line(e, f) -> vertical line(f, g) ->
               horizontal line(g, h) -> vertical line(h, e) -> close
  fix((10, -0.5)) e
  e distance(1) f
  f distance(1) g
}
bar := solid(std.solids.Prism(footprint, from: -0.6mm, to: 0.6mm))

swing := frame
swing turns(1) std.z
notch := solid({ p | some t in (-140deg, 30deg): p in bar at(under: swing) t })
notch cut part
