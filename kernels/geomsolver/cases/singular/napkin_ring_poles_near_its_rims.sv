// A napkin ring: the ball of radius 5 about the origin, made as its half disk turned a whole turn
// about the line through the origin along (3.1, 0, 4), so its poles lie on the sphere a hair
// outside the rims — less the bore of radius 3 along the z axis, a cutter through everything.
unit mm
use std
use std.solids
use std.solids (cut)

in std.front {
  o := point
  fix((0, 0)) o
  d := point
  fix((3.1, 4)) d
  construction tilt := line(o, d)
  north := point hint((3, 4))
  south := point hint((-3, -4))
  north coincident tilt
  south coincident tilt
  o distance(5mm) north
  o distance(5mm) south
  // the half disk on the side of +x: its arc from the south pole to the north, anticlockwise
  meridian := arc(center: o, start: south, end: north)
  construction centerline spindle := line(north, south)
}

in std.top {
  c := point
  fix((0, 0)) c
  hole := circle(center: c) hint(r: 3)
  radius(3mm) hole
}

part := solid(std.solids.Revolution(face(meridian, spindle), about: spindle))
bore := solid(std.solids.Through(face(hole)))
bore cut part
