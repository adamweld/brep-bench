// Two rods of radius 5 and length 30 crossing square, one along x and one along y, both centred on
// the origin; the part is the points in both.
unit mm
use std
use std.sets

west := point
fix((-15, 0, 0)) west
east := point
fix((15, 0, 0)) east
south := point
fix((0, -15, 0)) south
north := point
fix((0, 15, 0)) north
x_line := line(west, east)
y_line := line(south, north)

along_x := std.sets.Rod(x_line, r: 5mm)
along_y := std.sets.Rod(y_line, r: 5mm)
part := solid({ p | p in along_x; p in along_y })
