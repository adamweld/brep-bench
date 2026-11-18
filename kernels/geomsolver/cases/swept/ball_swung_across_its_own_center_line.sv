// The solid a ball fills as it is swung partway about an upright line through it: the ball of
// radius 2 about (1, 0, 0), turned rigidly 120 degrees about the z axis, right-handed (from +x
// toward +y).
unit mm
use std
use std.sets
use std.solids
use std.motion (turns)

o := point
fix((1, 0, 0)) o
ball := solid(std.sets.Ball(o, r: 2mm))

swing := frame
swing turns(1) std.z
part := solid({ p | some t in (0deg, 120deg): p in ball at(under: swing) t })
