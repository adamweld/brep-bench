// The half of the ball of radius 5 about the origin above the top plane: the points of the ball
// at or above the half-space's plane through the origin square to the z axis. Its rim, where the
// flat meets the ball, rounded to radius 1. Then the round deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the round runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face
// of the rounded hemisphere but the round's (its flat and the ball), read from its own faces.
unit mm
use std
use std.sets
use std.rounds (round)

o := point
fix((0, 0, 0)) o
down := point
fix((0, 0, -1)) down
z_down := line(o, down)

ball := std.sets.Ball(o, r: 5mm)
flat := std.sets.HalfSpace(o, z_down, d: 0mm)
dome := solid({ p | p in ball; p in flat })
dome.flat round(r: 1mm) dome.ball

part := solid({ p | p in dome.ball.side; p in dome.flat.side })
