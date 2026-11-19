// A napkin ring grown by 1: the points of the ball of radius 5 about the origin that are not within
// 3 of the z axis, and the points within 1 of that ring, `std.rounds.Grown`'s body.
unit mm
use std
use std.sets
use std.rounds

o := point
fix((0, 0, 0)) o
up := point
fix((0, 0, 1)) up
z_axis := line(o, up)

ball := std.sets.Ball(o, r: 5mm)
bore := std.sets.Cylinder(z_axis, r: 3mm)
ring := solid({ p | p in ball; p not in bore })
grown := std.rounds.Grown(ring, d: 1mm)
part := solid(grown.body)
