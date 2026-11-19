// A curved block, the ring sector between radii 8 and 12 about the z axis, z within 1, polar angle
// from 0 to 60 degrees: its section in the front plane (x, z) turned 60 degrees about the z axis.
// A ring lying flat: the torus whose tube, of radius 0.4, is centred on the circle of radius 1
// about the upright line through (10, 0) in the plane z = 0; its tube's circle drawn in the front
// plane and turned about that line. It is turned rigidly about the z axis from -140 to +30
// degrees, and everything it passes through cut from the block.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (turns)
use std.place (horizontal)

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
  o := point
  up := point hint((10, 1))
  fix((10, 0)) o
  o distance(1) up
  ring_axis := vertical line(o, up)
  m := point hint((11, 0))
  o distance(1, along: u) m
  o horizontal m
  tube := circle(center: m) hint(r: 0.4)
  radius(0.4) tube
}
ring := solid(std.solids.Revolution(face(tube), about: ring_axis))

swing := frame
swing turns(1) std.z
notch := solid({ p | some t in (-140deg, 30deg): p in ring at(under: swing) t })
notch cut part
