// Two balls of radius 5, one about the origin and one about the point 6 from it along x; the part
// is the first with the second in union.
unit mm
use std
use std.solids (union)
use std.sets
use std.place (right_of)

in std.front {
  a := point
  fix((0, 0)) a
  b := point hint((6, 0))
  b right_of(d: 6mm) a
}

other := solid(std.sets.Ball(b, r: 5mm))
part := solid(std.sets.Ball(a, r: 5mm))
other union part
