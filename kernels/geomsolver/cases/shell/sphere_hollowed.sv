// A ball of radius 5 about the origin hollowed to a wall 1 thick with no opening: `shell(t: …) X`
// keeps what lies within 1 of everything outside the ball, so a closed cavity, the ball of radius
// 4, stands inside it.
unit mm
use std
use std.sets
use std.rounds (shell)

o := point
fix((0, 0, 0)) o

part := solid(std.sets.Ball(o, r: 5mm))
shell(t: 1mm) part
