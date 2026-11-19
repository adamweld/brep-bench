// A tee of two cylinders crossing square: the main, of radius 5, on the x axis, its circle drawn
// about the origin of the side plane (y, z) and the prism over it from x = -15 to x = 15; the
// branch, of radius 3, parallel to the y axis on the line x = 0, z = 2, its circle drawn in the
// front plane (x, z) 2 up from the origin and the prism over it from y = -15 to y = 15. Both tops
// are at z = 5, where the walls touch. Their common part.
unit mm
use std
use std.solids
use std.solids (common)
use std.place (vertical)

in std.side {
  o := point
  fix((0, 0)) o
  main := circle(center: o) hint(r: 5)
  radius(5) main
}

in std.front {
  q := point
  fix((0, 0)) q
  c := point hint((0, 2))
  q distance(2mm, along: v) c
  q vertical c
  branch := circle(center: c) hint(r: 3)
  radius(3) branch
}

branch_rod := solid(std.solids.Prism(face(branch), from: -15mm, to: 15mm))
part := solid(std.solids.Prism(face(main), from: -15mm, to: 15mm))
branch_rod common part
