// A cube of side 10 about the origin, its square drawn about the origin of the front plane (x, z)
// and the prism over it from y = -5 to y = 5, united with a cylinder of radius 5 on the x axis, its
// circle drawn about the origin of the side plane (y, z) and the prism over it from x = -10 to
// x = 10. The cylinder's wall touches four of the cube's faces along their middle lines.
unit mm
use std
use std.shapes
use std.solids
use std.solids (union)

in std.front {
  o := point
  fix((0, 0)) o
  square := std.shapes.CenteredRectangle(o, w: 10mm, h: 10mm)
}

in std.side {
  c := point
  fix((0, 0)) c
  rim := circle(center: c) hint(r: 5)
  radius(5) rim
}

rod := solid(std.solids.Prism(face(rim), from: -10mm, to: 10mm))
part := solid(std.solids.Prism(square.loop, from: -5mm, to: 5mm))
rod union part
