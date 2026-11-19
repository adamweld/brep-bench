// A ball of radius 5 about the origin with its top cut flat: its points at most 3 above the top
// plane along the z axis, the flat the disc where that half-space's plane cuts it. Then the flat
// deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the flat runs the faces round it on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of every face of
// the cut ball but the flat (here only the ball's own), read from the cut ball's faces.
unit mm
use std
use std.sets

o := point
fix((0, 0, 0)) o
up := point
fix((0, 0, 1)) up
z_axis := line(o, up)

ball := std.sets.Ball(o, r: 5mm)
flat := std.sets.HalfSpace(o, z_axis, d: 3mm)
cut_ball := solid({ p | p in ball; p in flat })

part := solid({ p | p in cut_ball.ball.side })
