// Two halves of the ball of radius 5 about the origin, united: the first the points of the ball at
// or above the top plane (z >= 0), the second those at or beyond the side plane (x >= 0), each along
// its plane's normal.
unit mm
use std
use std.sets
use std.solids (union)

o := point
fix((0, 0, 0)) o

ball := std.sets.Ball(o, r: 5mm)
upper := solid({ p | p in ball; p distance(>= 0mm, along: n) std.top })
part := solid({ p | p in ball; p distance(>= 0mm, along: n) std.side })
upper union part
