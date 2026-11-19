// A curved block, the ring sector between radii 8 and 12 about the z axis, z within 2, polar angle
// from -30 to +30 degrees: its section in the front plane (x, z) turned about the z axis from -30
// to +30 degrees.
// A ball-end mill about the upright line through (10, 0): the lower half of the ball of radius 1
// about (10, 0, 0) under a cylinder of radius 1 up to z = 3, a turn of its half-section in the
// front plane. It is carried by a screw about the z axis that turns right-handed (from +x toward
// +y) and rises 3 a turn, from a turn of -60 degrees to one of +60, and everything it passes
// through cut from the block.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (screws)

in std.front {
  a := point
  b := point hint((12, -2))
  c := point hint((12, 2))
  d := point hint((8, 2))
  meridian := horizontal line(a, b) -> vertical line(b, c) ->
              horizontal line(c, d) -> vertical line(d, a) -> close
  fix((8, -2)) a
  a distance(4) b
  b distance(4) c
}
part := solid(std.solids.Revolution(meridian, about: std.z, from: -30deg, to: 30deg))

in std.front {
  tip := point
  k := point
  lip := point hint((11, 0))
  e2 := point hint((11, 3))
  e3 := point hint((10, 3))
  fix((10, -1)) tip
  fix((10, 0)) k
  fix((10, 3)) e3
  // the half ball, from its lowest point round to its widest circle, where the side goes on
  half := (ball := arc(center: k, start: tip, end: lip)) -> tangent vertical line(lip, e2) ->
          horizontal line(e2, e3) -> vertical (spindle := line(e3, tip)) -> close
  radius(1) ball
}
mill := solid(std.solids.Revolution(half, about: spindle))

climb := frame
climb screws(3mm) std.z
groove := solid({ p | some t in (-60deg, 60deg): p in mill at(under: climb) t })
groove cut part
