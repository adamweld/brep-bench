// A spindle torus: the disk of radius 5 about (3, 0) in the front plane (x, z) crosses the z axis,
// so the solid it fills turning about the axis is the turn of its part with x >= 0: the outer arc
// of its circle, from where it crosses the axis below, round through (8, 0), to where it crosses
// above, closed by the chord on the axis, turned a whole turn about the z axis.
unit mm
use std
use std.solids

in std.front {
  c := point
  fix((3, 0)) c
  south := point hint((0, -4))
  north := point hint((0, 4))
  fix(x == 0mm) south
  fix(x == 0mm) north
  outer := arc(center: c, start: south, end: north) hint(r: 5)
  radius(5mm) outer
  construction centerline chord := line(north, south)
}

part := solid(std.solids.Revolution(face(outer, chord), about: std.z))
