// The ball of radius 5 about the origin, less everything below the xy plane: the points of the
// ball at or above the top plane along its normal.
unit mm
use std
use std.sets

o := point
fix((0, 0, 0)) o

ball := std.sets.Ball(o, r: 5mm)
part := solid({ p | p in ball; p distance(>= 0mm, along: n) std.top })
