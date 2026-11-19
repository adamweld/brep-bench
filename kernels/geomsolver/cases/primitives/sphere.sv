// A ball of radius 5 about the origin: the points within 5 of it, as the case's set says.
unit mm
use std
use std.sets

o := point
fix((0, 0, 0)) o

part := solid(std.sets.Ball(o, r: 5mm))
