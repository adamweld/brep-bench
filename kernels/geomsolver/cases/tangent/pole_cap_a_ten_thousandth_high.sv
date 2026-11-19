// The cap of the ball of radius 5 about the origin above the plane z = 4.9999, 10^-4 below its top:
// the points of the ball at least 4.9999 along the top plane's normal.
unit mm
use std
use std.sets

o := point
fix((0, 0, 0)) o

ball := std.sets.Ball(o, r: 5mm)
part := solid({ p | p in ball; p distance(>= 4.9999mm, along: n) std.top })
