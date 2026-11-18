// A cone cut aslant through its apex: the cone with its apex at (0, 0, 10) opening down the z axis
// through the rim point (5, 0, 0), above the top plane — a cone on the disk of radius 5 — on the
// axis's side of the plane 4x + z = 10, the plane through the apex and the line x = 2.5 of the
// floor.
unit mm
use std
use std.sets

apex := point
fix((0, 0, 10)) apex
foot := point
fix((0, 0, 0)) foot
rim := point
fix((5, 0, 0)) rim
a := point
fix((2.5, 0, 0)) a
b := point
fix((2.5, 1, 0)) b

spine := line(apex, foot)
cone := std.sets.Cone(spine, half: hint(27deg))
rim in boundary(cone)

// the plane over the line on the floor and the slope up to the apex; its normal, along × slope,
// points away from the axis, and the cone is kept behind it
along := line(a, b)
slope := line(a, apex)
cut := plane(u: along, v: slope)
part := solid({ p | p in cone; p distance(>= 0mm, along: n) std.top; p distance(<= 0mm, along: n) cut })
