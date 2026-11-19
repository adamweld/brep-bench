// Two balls of radius 5, one about the origin and one about (9.999999, 0, 0), overlapping to a depth
// of a millionth: the lens common to both.
unit mm
use std
use std.sets
use std.solids (common)

o := point
fix((0, 0, 0)) o
c := point
fix((9.999999, 0, 0)) c

second := solid(std.sets.Ball(c, r: 5mm))
part := solid(std.sets.Ball(o, r: 5mm))
second common part
