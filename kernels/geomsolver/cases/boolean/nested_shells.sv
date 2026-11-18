// Three bodies nested in each other's hollows, each Boolean a named body: the ball of radius 10
// about the origin less the ball of radius 8 about (1, 0, 0); the ball of radius 6 about
// (1, 1, 0) less the ball of radius 4 about (1, 1, 1); and the part, the first shell with the
// second and the ball of radius 2 about (1, 1.5, 1.5) in union with it.
unit mm
use std
use std.solids (cut, union)
use std.sets

c10 := point
fix((0, 0, 0)) c10
c8 := point
fix((1, 0, 0)) c8
c6 := point
fix((1, 1, 0)) c6
c4 := point
fix((1, 1, 1)) c4
c2 := point
fix((1, 1.5, 1.5)) c2

hollow8 := solid(std.sets.Ball(c8, r: 8mm))
outer := solid(std.sets.Ball(c10, r: 10mm))
hollow8 cut outer

hollow4 := solid(std.sets.Ball(c4, r: 4mm))
middle := solid(std.sets.Ball(c6, r: 6mm))
hollow4 cut middle

core := solid(std.sets.Ball(c2, r: 2mm))
part := solid(outer)
middle union part
core union part
