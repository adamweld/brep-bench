// A ball of radius 15 about the origin, united with a cone on the z axis, its apex at (0, 0, -25)
// and its flat base the disk of radius 12 at z = -9: the cone the half-section, a right triangle
// drawn in the front plane (x, z) from the axis, turned a whole turn about z. Its side is tangent
// to the sphere along the rim of its base.
unit mm
use std
use std.sets
use std.solids
use std.solids (union)

in std.front {
  apex := point
  axis_top := point hint((0, -9))
  rim := point hint((12, -9))
  fix((0, -25)) apex
  apex distance(16) axis_top
  axis_top distance(12) rim
  half := line(apex, rim) -> horizontal line(rim, axis_top) -> vertical line(axis_top, apex) -> close
}

cone := solid(std.solids.Revolution(half, about: std.z))
part := solid(std.sets.Ball(std.origin, r: 15mm))
cone union part
