// A cone halved through its apex: the cone with its apex at (0, 0, 10) opening down the z axis
// through the rim point (5, 0, 0), above the top plane — a cone on the disk of radius 5 — and on or
// in front of the side plane (y, z), along its normal +x.
unit mm
use std
use std.sets

apex := point
fix((0, 0, 10)) apex
foot := point
fix((0, 0, 0)) foot
rim := point
fix((5, 0, 0)) rim

spine := line(apex, foot)
cone := std.sets.Cone(spine, half: hint(27deg))
rim in boundary(cone)
part := solid({ p | p in cone; p distance(>= 0mm, along: n) std.top; p distance(>= 0mm, along: n) std.side })
