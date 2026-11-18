// A ball cut aslant through its pole: the points of the ball of radius 5 about the origin on the
// centre's side of the plane x + z = 5, the plane through (5, 0, 0) and the pole (0, 0, 5) that
// runs along y.
unit mm
use std
use std.sets

o := point
fix((0, 0, 0)) o
a := point
fix((5, 0, 0)) a
pole := point
fix((0, 0, 5)) pole
b := point
fix((5, 1, 0)) b

// the plane over its slope and a line along y; its normal, slope × along, points back toward the
// centre
slope := line(a, pole)
along := line(a, b)
cut := plane(u: slope, v: along)
ball := std.sets.Ball(o, r: 5mm)
part := solid({ p | p in ball; p distance(>= 0mm, along: n) cut })
