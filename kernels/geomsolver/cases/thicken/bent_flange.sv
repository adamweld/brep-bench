// A bent sheet-metal flange — the floor from (−10, 0) to the origin, the bend a quarter circle of
// radius 2 about (0, 2) to (2, 2), the wall up to (2, 12), in the xz plane, swept along y from 0
// to 8 — thickened by 1 outside its bend. Solvent's thickening of a sheet, a developable panel's
// band, refuses a flat panel ("the rulings fold" between two parallel rails, here along the floor),
// so the thickened sheet is drawn as its section: in the front plane (x, z) the profile and the
// profile moved 1 along its outward normal — the floor's line 1 below it, the bend's arc concentric
// with it of radius 3, the wall's line 1 beyond it — closed by the strips their ends sweep, and
// the prism over it from y = 0 to 8.
unit mm
use std
use std.solids

l := 10mm      // the floor's and the wall's length
R := 2mm       // the bend's radius
t := 1mm
w := 8mm       // the sheet's width along y

in std.front {
  o := point                               // where the floor meets the bend
  fix((0mm, 0mm)) o
  c := point hint((0, 2))                  // the bend's axis
  o distance(R, along: v) c
  o level(u) c
  p0 := point hint((-10, 0))
  o distance(-l, along: u) p0
  o level(v) p0
  p2 := point hint((2, 2))                 // where the bend meets the wall
  c level(v) p2
  p3 := point hint((2, 12))
  p2 distance(l, along: v) p3
  p2 level(u) p3
  // the profile moved t out
  q0 := point hint((-10, -1))
  p0 distance(-t, along: v) q0
  p0 level(u) q0
  q1 := point hint((0, -1))
  o distance(-t, along: v) q1
  o level(u) q1
  q2 := point hint((3, 2))
  c level(v) q2
  q3 := point hint((3, 12))
  p3 distance(t, along: u) q3
  p3 level(v) q3
  floor := line(p0, o)
  bend := arc(center: c, start: o, end: p2)
  wall := line(p2, p3)
  wall_end := line(p3, q3)
  wall_out := line(q3, q2)
  bend_out := arc(center: c, start: q1, end: q2)
  floor_out := line(q1, q0)
  floor_end := line(q0, p0)
  section := face(floor, bend, wall, wall_end, wall_out, bend_out, floor_out, floor_end)
}
// the front's normal is −y, so the material behind the section runs along +y
part := solid(std.solids.Prism(section, from: -w, to: 0mm))
