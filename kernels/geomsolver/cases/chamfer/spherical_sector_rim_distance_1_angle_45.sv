// A spherical sector: the part of a ball of radius 5 about the origin inside the cone about the z
// axis with its apex at the origin and half angle α, cos α = 3/5, whose side meets the ball in the
// sector's rim. The rim chamfered by a distance and an angle: 1 down the cone from the rim, the
// chamfer making 45° with the cone's side, turned toward the axis, until it meets the ball. The
// chamfer is the cone that line sweeps about the axis.
//
// The library's chamfer takes legs, not an angle, and its flat bows beside a ball (the distances
// from the two surfaces, each over the other's leg, summing to one). So the chamfered half-section
// is drawn in the front plane (x the radius, z up) with the chamfer as one of its sides, and turned
// about the z axis: up the cone's side from the apex to `foot`, 1 short of the rim `e`; the chamfer
// at 45° to the side, up to `q` on the ball; the ball's arc up to the pole; down the axis.
unit mm
use std
use std.solids

in std.front {
  o := point
  fix((0, 0)) o
  pole := point hint((0, 5))
  construction up := line(o, pole)
  e := point hint((4, 3))
  o distance(5mm) e
  foot := point hint((3.2, 2.4))
  q := point hint((3.38, 3.68))
  half := (side := line(o, foot)) -> (chamfer := line(foot, q)) ->
          (dome := arc(center: o, start: q, end: pole)) -> vertical line(pole, o) -> close
  side angle(acos(3 / 5)) up
  e coincident side
  foot distance(1mm) e
  side angle(45deg) chamfer
  radius(5mm) dome
}

part := solid(std.solids.Revolution(half, about: std.z))
