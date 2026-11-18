// A capsule's end as a turn of its half-section: in the front plane (x, z), the base out to
// radius 5, the side up to z = 10, the quarter circle of radius 5 about (0, 10) over to the axis
// and the axis back down, turned a whole turn about the z axis: a cylinder capped by a hemisphere
// joined to it tangentially. Shelled to a wall 1 thick and open through its hemisphere,
// `part.dome`: the side's wall runs on past its face's end up to the opening.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.front {
  o := point
  fix((0, 0)) o
  b := point hint((5, 0))
  c := point hint((5, 10))
  m := point hint((0, 10))
  n := point hint((0, 15))
  horizontal (base := line(o, b))
  vertical (side := line(b, c))
  (dome := arc(center: m, from: c, to: n))
  vertical (axle := line(n, o))
  m coincident axle
  o distance(5) b
  o distance(10, along: up) m
  m level(up) c
}

part := solid(std.solids.Revolution(face(base, side, dome, axle), about: std.z))
part.dome shell(t: 1mm) part
