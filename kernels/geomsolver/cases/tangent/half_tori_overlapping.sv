// Two halves of the torus about the z axis whose tube, of radius 3, is centred 10 out, united. The
// torus is its tube's circle drawn in the front plane (x, z) 10 out along x and turned a whole turn
// about z; the second is turned from a quarter turn on, so the faces that coincide are not split in
// the same places. The first half is what the torus has in common with a block over z >= 0, the
// second what it has in common with a block over x >= 0, each block reaching past the torus on
// every other side.
unit mm
use std
use std.shapes
use std.solids
use std.solids (union, common)

in std.front {
  c := point
  fix((10, 0)) c
  tube := circle(center: c) hint(r: 3)
  radius(3) tube
}

in std.top {
  o := point
  fix((0, 0)) o
  plan := std.shapes.CenteredRectangle(o, w: 30mm, h: 30mm)
}

in std.side {
  q := point
  fix((0, 0)) q
  elevation := std.shapes.CenteredRectangle(q, w: 30mm, h: 8mm)
}

above := solid(std.solids.Prism(plan.loop, from: 0mm, to: 4mm))
beyond := solid(std.solids.Prism(elevation.loop, from: 0mm, to: 14mm))

upper := solid(std.solids.Revolution(face(tube), about: std.z))
above common upper
eastern := solid(std.solids.Revolution(face(tube), about: std.z, from: 90deg, to: 450deg))
beyond common eastern

part := solid(upper)
eastern union part
