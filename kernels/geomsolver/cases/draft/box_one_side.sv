// The box from the origin to (20, 12, 10), its side x = 20 drafted 10° with the pull along +z about
// the floor: the side turns about its bottom edge until it leans 10° from the z direction, in
// toward -x as it rises, and no other face is drafted.
//
// Solvent's draft (`std.solids.Drafted`) leans every side of a section from the section's plane, so
// it cannot draft one side of the box alone. The drafted box is drawn instead as its section in the
// front plane (x, z) — the floor 20 long, the drafted side rising from its end at 100° to it, 10°
// past square, the top 10 up, the back square to the floor — and the prism over it from y = 0 to
// y = 12 (the front's normal is -y).
unit mm
use std
use std.solids

in std.front {
  o := point
  fix((0, 0)) o
  b := point hint((20, 0))
  c := point hint((18.2, 10))
  d := point hint((0, 10))
  section := horizontal (floor := line(o, b)) -> (drafted := line(b, c)) ->
             horizontal (top := line(c, d)) -> vertical (back := line(d, o)) -> close
  o distance(20) b
  o distance(10) d
  floor angle(90deg + 10deg) drafted
}

part := solid(std.solids.Prism(section, from: -12mm, to: 0mm))
