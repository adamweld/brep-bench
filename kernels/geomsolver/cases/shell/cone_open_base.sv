// A solid cone as a turn of its half-section: in the front plane (x, z), the triangle from the axis
// out to radius 6 at z = 0 and up to the apex at z = 8, turned a whole turn about the z axis.
// Shelled to a wall 1 thick and open through its base, `part.base`: the side moved in 1 is the cone
// of the same half-angle, its apex moved down the axis.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.front {
  o := point
  fix((0, 0)) o
  r := point hint((6, 0))
  apex := point hint((0, 8))
  horizontal (base := line(o, r)) -> (side := line(r, apex)) -> vertical (axle := line(apex, o)) -> close
  o distance(6) r
  o distance(8) apex
}

part := solid(std.solids.Revolution(face(base, side, axle), about: std.z))
part.base shell(t: 1mm) part
