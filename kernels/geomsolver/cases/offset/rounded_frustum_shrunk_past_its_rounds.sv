// A cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, both its circular edges
// rounded to radius 1.5, shrunk by 2: its half-section drawn in the front plane (x, z) and turned a
// whole turn about the z axis, the round put on, and the points of it at least 2 from its outside,
// `std.rounds.Grown`'s body for a negative reach.
unit mm
use std
use std.solids
use std.rounds
use std.rounds (round)

in std.front {
  a := point
  b := point hint((6, 0))
  c := point hint((3, 6))
  d := point hint((0, 6))
  horizontal (bottom := line(a, b)) -> (side := line(b, c)) -> horizontal (top := line(c, d)) ->
  vertical (axis_run := line(d, a)) -> close
  fix((0, 0)) a
  a distance(6) b
  a distance(6) d
  d distance(3) c
  half := face(bottom, side, top, axis_run)
}

frustum := solid(std.solids.Revolution(half, about: std.z))
frustum.side round(r: 1.5mm) frustum
shrunk := std.rounds.Grown(frustum, d: -2mm)
part := solid(shrunk.body)
