// Two balls meeting through a pole of each: the balls of radius 5 about the origin and about
// (4.8, 0, 3.6), 6 apart, united.
unit mm
use std
use std.sets
use std.solids (union)

o := point
fix((0, 0, 0)) o
c := point
fix((4.8, 0, 3.6)) c

first := solid(std.sets.Ball(o, r: 5mm))
second := solid(std.sets.Ball(c, r: 5mm))
part := solid(first)
second union part
