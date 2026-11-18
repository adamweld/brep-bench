// The solid a ball fills as it is screwed round a helix tighter than itself: the ball of radius 2
// about (1.5, 0, 0), carried by a screw about the z axis that turns right-handed (from +x toward
// +y) and rises 3 a turn, for two whole turns.
unit mm
use std
use std.sets
use std.solids
use std.motion (screws)

o := point
fix((1.5, 0, 0)) o
ball := solid(std.sets.Ball(o, r: 2mm))

climb := frame
climb screws(3mm) std.z
part := solid({ p | some t in (0deg, 720deg): p in ball at(under: climb) t })
