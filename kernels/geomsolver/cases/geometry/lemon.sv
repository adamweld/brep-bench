// A lemon: the segment the z axis cuts from the disk of radius 5 about (-3, 0) in the front plane
// (x, z), on the side x >= 0 — the circle's arc from where it crosses the axis below, through
// (2, 0), to where it crosses above, closed by the chord on the axis — turned a whole turn about
// the z axis.
unit mm
use std
use std.solids

in std.front {
  c := point
  fix((-3, 0)) c
  south := point hint((0, -4))
  north := point hint((0, 4))
  fix(x == 0mm) south
  fix(x == 0mm) north
  rind := arc(center: c, start: south, end: north) hint(r: 5)
  radius(5mm) rind
  construction centerline chord := line(north, south)
}

part := solid(std.solids.Revolution(face(rind, chord), about: std.z))
