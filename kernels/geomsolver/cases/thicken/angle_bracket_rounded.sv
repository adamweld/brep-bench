// A creased sheet — the floor, 10 × 8 in the xy plane, and the wall, 8 × 10 in the yz plane,
// meeting at a right angle along the y axis — thickened by 1 outside its crease with the corner
// rounded. Solvent's thickening of a sheet, a developable panel's band, refuses a flat panel ("the
// rulings fold" between two parallel rails), so the thickened sheet is drawn as its section: in
// the front plane (x, z) the floor and the wall, each moved 1 along its outward normal, the gap
// they leave at the crease closed by the quarter circle of radius 1 about it, and the strips their
// ends sweep — and the prism over it from y = 0 to 8.
unit mm
use std
use std.solids

l := 10mm
t := 1mm
w := 8mm

in std.front {
  crease := point
  fix((0mm, 0mm)) crease
  f0 := point hint((10, 0))
  f1 := point hint((10, -1))
  k0 := point hint((0, -1))
  k1 := point hint((-1, 0))
  w1 := point hint((-1, 10))
  w0 := point hint((0, 10))
  crease distance(l, along: u) f0
  crease level(v) f0
  crease distance(-t, along: v) k0
  crease level(u) k0
  crease level(v) k1                                     // on the quarter's circle, level with it
  f0 distance(-t, along: v) f1
  f0 level(u) f1
  w0 distance(-t, along: u) w1
  w0 level(v) w1
  crease distance(l, along: v) w0
  crease level(u) w0
  floor := line(crease, f0)
  floor_end := line(f0, f1)
  floor_moved := line(f1, k0)
  corner := arc(center: crease, start: k1, end: k0)     // the quarter about the crease
  wall_moved := line(k1, w1)
  wall_end := line(w1, w0)
  wall := line(w0, crease)
  section := face(floor, floor_end, floor_moved, corner, wall_moved, wall_end, wall)
}
part := solid(std.solids.Prism(section, from: -w, to: 0mm))
