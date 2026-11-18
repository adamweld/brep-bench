// A hemisphere with its rim rounded, shrunk by 3/2: the points of the ball of radius 5 about the
// origin at or above the top plane, the rim where the two bounds meet rounded to radius 1 (one more
// bound of the region, `std.rounds.Rounded`), and the points of it at least 3/2 from its outside,
// `std.rounds.Grown`'s body for a negative reach.
unit mm
use std
use std.sets
use std.rounds

o := point
fix((0, 0, 0)) o

ball := std.sets.Ball(o, r: 5mm)
above := { q | q distance(>= 0mm, along: n) std.top }
dome := solid({ p | p in ball; p in above; p in std.rounds.Rounded(ball, above, r: 1mm) })
shrunk := std.rounds.Grown(dome, d: -1.5mm)
part := solid(shrunk.body)
