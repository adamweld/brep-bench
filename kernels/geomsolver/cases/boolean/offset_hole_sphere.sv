// A ball of radius 20 about the origin, pierced parallel to y: the hole's circle, radius 6, drawn
// in the front plane (x, z) 5 out along x, and the cutter through it along the plane's normal, y,
// cut from the ball; it runs through the whole ball, however far.
unit mm
use std
use std.solids
use std.solids (cut)
use std.sets
use std.place (right_of)

in std.front {
  ax := point hint((5, 0))
  ax right_of(d: 5mm) std.origin
  rim := circle(center: ax) hint(r: 6)
  radius(6mm) rim
}

drill := solid(std.solids.Through(face(rim)))
part := solid(std.sets.Ball(std.origin, r: 20mm))
drill cut part
