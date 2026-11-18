// A cup's skin — a sheet of revolution about the z axis, a floor, a bend and a wall,
// its faces meeting tangentially: the floor, the disk of radius 4 at z = 0; the bend, the quarter
// of a torus whose tube of radius 2 runs about the circle of radius 4 at z = 2; and the wall, the
// cylinder of radius 6 up to z = 10 — thickened by 3 to the inside, past the bend's radius.
// Solvent's thickening of a sheet, a developable panel's band, takes a ruled panel between two
// rails, and this sheet is neither ruled nor developable (its bend is a torus), so the thickened
// sheet is drawn as its section and turned: in the front plane (ρ out from the axis, z up) the
// sheet's profile, and the floor moved 3 up and the wall moved 3 in, meeting in their concave
// corner — the bend's own moved face, turned inside out, bounds nothing — closed down the axis;
// the section turned a whole turn about z.
unit mm
use std
use std.solids

c := 4mm      // the floor's radius
r := 2mm      // the bend's radius
H := 10mm      // the wall's top
t := 3mm      // the thickness

in std.front {
  o := point
  fix((0mm, 0mm)) o
  f := point hint((4, 0))                  // where the floor meets the bend
  o distance(c, along: u) f
  o level(v) f
  k := point hint((4, 2))                  // the bend's tube centre
  f distance(r, along: v) k
  f level(u) k
  g := point hint((6, 2))                  // where the bend meets the wall
  k level(v) g
  h := point hint((6, 10))
  g distance(H - r, along: v) h
  g level(u) h
  // the wall moved t in and the floor t up, to where they meet, and the floor's on the axis
  m := point hint((3, 10))
  h distance(-t, along: u) m
  h level(v) m
  n := point hint((3, 3))
  m level(u) n
  o distance(t, along: v) n
  a := point hint((0, 3))
  o distance(t, along: v) a
  o level(u) a
  floor := line(o, f)
  bend := arc(center: k, start: f, end: g)
  wall := line(g, h)
  rim := line(h, m)
  wall_in := line(m, n)
  floor_in := line(n, a)
  spine := line(a, o)
  section := face(floor, bend, wall, rim, wall_in, floor_in, spine)
}
part := solid(std.solids.Revolution(section, about: std.z))
