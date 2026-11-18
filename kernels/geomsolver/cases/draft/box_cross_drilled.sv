// The box from the origin to (30, 20, 20), drilled right through along x by a hole of radius 4
// about the line y = 10, z = 10, and then its two faces x = 0 and x = 30, where the hole comes
// out, drafted 8° with the pull along +z about the floor: each turns about its bottom edge until it
// leans 8° from the z direction, inward as it rises, and nothing else is drafted. The hole's wall
// stays the cylinder, trimmed by the drafted faces.
//
// Solvent's draft (`std.solids.Drafted`) leans every side of a section from the section's plane, so
// it cannot draft two of the box's sides alone. The drafted box is drawn instead as its section in
// the front plane (x, z) — the floor 30 long, each drafted side rising from its end at 8° past
// square to it, the top 20 up — and the prism over it from y = 0 to y = 20 (the front's normal is
// -y). The hole is a circle drawn in the side plane (y, z) and the cutter through it, cut from the
// box.
unit mm
use std
use std.solids
use std.solids (cut)

in std.front {
  o := point
  fix((0, 0)) o
  b := point hint((30, 0))
  c := point hint((27.2, 20))
  d := point hint((2.8, 20))
  section := horizontal (floor := line(o, b)) -> (east := line(b, c)) ->
             horizontal (top := line(c, d)) -> (west := line(d, o)) -> close
  o distance(30) b
  o distance(20, along: v) c
  floor angle(90deg + 8deg) east
  west angle(90deg + 8deg) floor
}
part := solid(std.solids.Prism(section, from: -20mm, to: 0mm))

in std.side {
  axis_point := point
  fix((10, 10)) axis_point
  bore := circle(center: axis_point) hint(r: 4)
  radius(4mm) bore
}
hole := solid(std.solids.Through(face(bore)))
hole cut part
