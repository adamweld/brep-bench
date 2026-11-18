// A lens of two unequal balls: one of radius 5.511 about the origin, one of radius 3.317 about
// (4.231, 3.467, 2.653), off every axis. The part is the first kept where it is in common with
// the second.
unit mm
use std
use std.solids (common)
use std.sets

a := point
fix((0, 0, 0)) a
b := point
fix((4.231, 3.467, 2.653)) b

other := solid(std.sets.Ball(b, r: 3.317mm))
part := solid(std.sets.Ball(a, r: 5.511mm))
other common part
