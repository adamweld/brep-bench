// The bent sheet of `thicken/bent-flange` — the floor from (−10, 0) to the origin, the bend a
// quarter circle of radius 2 about (0, 2) to (2, 2), the wall up to (2, 12), in the xz plane, swept
// along y from 0 to 8 — thickened by 3 inside its bend, more than the bend's radius. Solvent's
// thickening of a sheet, a developable panel's band, refuses a flat panel ("the rulings fold"
// between two parallel rails, here along the floor), so the thickened sheet is drawn as its
// section: in the front plane (x, z) the profile, and the floor and the wall moved 3 along their
// inward normals, meeting in the concave corner at (−1, 3) — the bend's own moved face, turned
// inside out, bounds nothing — closed by the strips their ends sweep; and the prism over it from
// y = 0 to 8.
unit mm
use std
use std.solids

l := 10mm
R := 2mm
t := 3mm
w := 8mm

in std.front {
  o := point
  fix((0mm, 0mm)) o
  c := point hint((0, 2))
  o distance(R, along: v) c
  o level(u) c
  p0 := point hint((-10, 0))
  o distance(-l, along: u) p0
  o level(v) p0
  p2 := point hint((2, 2))
  c level(v) p2
  p3 := point hint((2, 12))
  p2 distance(l, along: v) p3
  p2 level(u) p3
  // the floor moved t up and the wall t in, to where they meet
  q3 := point hint((-1, 12))
  p3 distance(-t, along: u) q3
  p3 level(v) q3
  k := point hint((-1, 3))
  q3 level(u) k
  p0 distance(t, along: v) k
  q0 := point hint((-10, 3))
  p0 distance(t, along: v) q0
  p0 level(u) q0
  floor := line(p0, o)
  bend := arc(center: c, start: o, end: p2)
  wall := line(p2, p3)
  wall_end := line(p3, q3)
  wall_in := line(q3, k)
  floor_in := line(k, q0)
  floor_end := line(q0, p0)
  section := face(floor, bend, wall, wall_end, wall_in, floor_in, floor_end)
}
part := solid(std.solids.Prism(section, from: -w, to: 0mm))
