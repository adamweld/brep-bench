// A ball halved through its poles: the points of the ball of radius 5 about the origin on or in
// front of the side plane (y, z), along its normal +x.
unit mm
use std
use std.sets

o := point
fix((0, 0, 0)) o

ball := std.sets.Ball(o, r: 5mm)
part := solid({ p | p in ball; p distance(>= 0mm, along: n) std.side })
