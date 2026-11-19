// A torus about the z axis, its tube's circle of radius 3 drawn in the front plane (x, z) 10 out
// along x and turned a whole turn about z from 45° to 405°, so that where its turn starts is off
// the points of contact, united with a square slab: its square drawn about the origin of the top
// plane (x, y), 26 on a side, and the prism over it from z = -1 to z = 1. The torus's outer equator
// touches each of the slab's four sides at its middle.
unit mm
use std
use std.shapes
use std.solids
use std.solids (union)

in std.front {
  c := point
  fix((10, 0)) c
  tube := circle(center: c) hint(r: 3)
  radius(3) tube
}

in std.top {
  o := point
  fix((0, 0)) o
  square := std.shapes.CenteredRectangle(o, w: 26mm, h: 26mm)
}

torus := solid(std.solids.Revolution(face(tube), about: std.z, from: 45deg, to: 405deg))
part := solid(std.solids.Prism(square.loop, from: -1mm, to: 1mm))
torus union part
