// The upper half of the ball of radius 5 about the origin, shelled outward to a wall 0.5 thick and
// open at its dome: the half ball's one face kept, its flat on the top plane, is the inside of the
// wall, and the outside is that face moved 0.5 outward, down along the plane's normal, run on until
// it meets the opening's surface, the sphere: the wall is the ball's material between the flat and
// the flat moved.
//
// `std.rounds`' `shell` hollows a body inward only, so the outward wall is said as its set: the
// points of the ball on the far side of the flat's plane, within 0.5 of it along its normal.
unit mm
use std
use std.sets

o := point
fix((0, 0, 0)) o

ball := std.sets.Ball(o, r: 5mm)
part := solid({ p |
  p in ball
  p distance(<= 0mm, along: n) std.top
  p distance(>= -0.5mm, along: n) std.top
})
