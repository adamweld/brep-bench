// A helical pipe with rounded ends: a ball of radius 1 rolled along the helix of radius 10 about
// the z axis that rises 5 a turn: the ball about (10, 0, 0) carried by the screw about the z axis,
// right-handed and climbing 5 a turn, for three turns. The solid is everything the ball passes
// through, so its ends are the ball's halves.
unit mm
use std
use std.sets
use std.motion (screws)

o := point
fix((0, 0, 0)) o
up := point
fix((0, 0, 1)) up
spindle := line(o, up)

start := point
fix((10, 0, 0)) start
ball := solid(std.sets.Ball(start, r: 1mm))

climb := frame
climb screws(5mm) spindle
part := solid({ p | some t in (0deg, 1080deg): p in ball at(under: climb) t })
