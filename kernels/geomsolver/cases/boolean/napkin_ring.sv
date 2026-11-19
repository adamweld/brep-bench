// A napkin ring, as the case's set says: the points of the ball of radius 5 about the origin that
// are not within 3 of the z axis.
unit mm
use std
use std.sets

o := point
fix((0, 0, 0)) o
up := point
fix((0, 0, 1)) up
z_axis := line(o, up)

ball := std.sets.Ball(o, r: 5mm)
bore := std.sets.Cylinder(z_axis, r: 3mm)
part := solid({ p | p in ball; p not in bore })
