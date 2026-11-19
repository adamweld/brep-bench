// A frustum on the z axis widening upward from the disc of radius 2 at z = 0 to radius 4 at
// z = 3, capped by the part of the ball of radius 5 about the origin above z = 3; the ball meets
// the frustum's cone in its rim, a convex edge. The rim chamfered by a distance and an angle: 2
// down the cone from the rim, the chamfer making 45° with the cone's side, turned toward the
// axis, until it meets the ball. The chamfer is the cone that line sweeps about the axis.
//
// The library's chamfer takes legs, not an angle, and its flat bows beside a ball (the distances
// from the two surfaces, each over the other's leg, summing to one). So the chamfered half-section
// is drawn in the front plane (x the radius, z up) with the chamfer as one of its sides, and turned
// about the z axis: across the floor to its rim `b`; up the cone's side to `foot`, 2 short of the
// rim `e`; the chamfer at 45° to the side, up to `q` on the ball; the ball's arc to the pole;
// down the axis.
unit mm
use std
use std.solids

in std.front {
  o := point
  fix((0, 0)) o
  b := point hint((2, 0))
  e := point hint((4, 3))
  foot := point hint((2.89, 1.34))
  q := point hint((2.27, 4.46))
  pole := point hint((0, 5))
  o distance(2mm) b
  o distance(5mm) e
  o distance(3mm, along: v) e
  half := horizontal line(o, b) -> (side := line(b, foot)) -> (chamfer := line(foot, q)) ->
          (dome := arc(center: o, start: q, end: pole)) -> vertical line(pole, o) -> close
  e coincident side
  foot distance(2mm) e
  side angle(45deg) chamfer
  radius(5mm) dome
}

part := solid(std.solids.Revolution(half, about: std.z))
