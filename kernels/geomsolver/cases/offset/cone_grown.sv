// A cone grown by 1: its half-section drawn in the front plane (x, z), the triangle from the axis
// out to radius 3 at z = 0 and up to the apex at (0, 4), turned a whole turn about the z axis; and
// the points within 1 of it, `std.rounds.Grown`'s body.
unit mm
use std
use std.solids
use std.rounds

in std.front {
  o := point
  rim := point hint((3, 0))
  apex := point hint((0, 4))
  half := horizontal line(o, rim) -> line(rim, apex) -> vertical line(apex, o) -> close
  fix((0, 0)) o
  o distance(3) rim
  o distance(4) apex
}

cone := solid(std.solids.Revolution(half, about: std.z))
grown := std.rounds.Grown(cone, d: 1mm)
part := solid(grown.body)
