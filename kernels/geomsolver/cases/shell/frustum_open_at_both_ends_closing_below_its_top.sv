// A solid cone frustum as a turn of its half-section: in the front plane (x, z), the quadrilateral
// from the axis out to radius 6 at z = 0 and radius 3 at z = 4, turned a whole turn about the z
// axis. Shelled to a wall 2.7 thick and open through both its base, `part.base`, and its top,
// `part.top`: the side moved in meets the base's plane and closes to its apex below the top, so the
// cavity opens through the base only.
//
// `std.rounds`' `A shell(t: …) B` takes one face path as its opening, so with two openings its
// own words are written out here: the cavity is the frustum's inside less the wall within 2.7 of
// the one face kept, the side, and it is cut from the frustum.
unit mm
use std
use std.solids
use std.solids (cut)

in std.front {
  o := point
  fix((0, 0)) o
  b := point hint((6, 0))
  c := point hint((3, 4))
  d := point hint((0, 4))
  horizontal (base := line(o, b)) -> (side := line(b, c)) -> horizontal (top := line(c, d)) ->
    vertical (axle := line(d, o)) -> close
  o distance(6) b
  o distance(4) d
  d distance(3) c
}

part := solid(std.solids.Revolution(face(base, side, top, axle), about: std.z))
within := { q | q in part }
cavity := solid(within)
repeat w in part {
  when w != part.base and w != part.top {
    wall := { p | p distance(<= 2.7mm) w }
    walled := solid(wall)
    cavity and { p | p not in walled }
  }
}
cavity cut part
