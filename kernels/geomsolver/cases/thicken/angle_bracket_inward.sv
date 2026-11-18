// A creased sheet — the floor, 10 × 8 in the xy plane, and the wall, 8 × 10 in the yz plane,
// meeting at a right angle along the y axis — thickened by 1 inside its crease, above the floor
// and in front of the wall, the moved faces trimmed where they cross. Solvent's thickening of a
// sheet, a developable panel's band, refuses a flat panel ("the rulings fold" between two parallel
// rails), so the thickened sheet is drawn as its section: in the front plane (x, z) the floor and
// the wall, each moved 1 along its inward normal and trimmed where the two cross, closed by the
// strips their ends sweep — the L through (0, 0), (10, 0), (10, 1), (1, 1), (1, 10), (0, 10) — and
// the prism over it from y = 0 to 8.
unit mm
use std
use std.solids

l := 10mm
t := 1mm
w := 8mm

in std.front {
  crease := point
  fix((0mm, 0mm)) crease
  section := step(u: l) (floor := line(crease)) -> step(dv: t) (floor_end := line) ->
             step(u: t) (floor_moved := line) -> step(v: l) (wall_moved := line) ->
             step(u: 0mm) (wall_end := line) -> (wall := line(p2: crease)) -> close
}
part := solid(std.solids.Prism(section, from: -w, to: 0mm))
