// A ball of radius 5 about the origin standing in a conical seat: a plate, the cylinder of radius
// 8 on the z axis from z = -6 to z = 5, with everything above the cone z = 1 + ρ/2 hollowed out of
// its top, and the ball joined to it. The seat's cone meets the ball in a concave edge. The edge
// chamfered by a distance and an angle: 3 up the seat's cone from it, the chamfer making 15°
// with the cone, rising from it back toward the ball until it first meets it, filling the groove.
// The chamfer is the cone that line sweeps about the axis.
//
// The library's chamfer takes legs, not an angle, and its flat bows beside a ball (the distances
// from the two surfaces, each over the other's leg, summing to one). So the chamfered half-section
// is drawn in the front plane (x the radius, z up) with the chamfer as one of its sides, and turned
// about the z axis: across the plate's underside and up its rim; down the seat's cone, the line
// from the rim through the cone's apex on the axis at z = 1, to `foot`, 3 short of the edge `e`;
// the chamfer at 15° to the cone, up to `q` on the ball; the ball's arc to the pole; down the
// axis.
unit mm
use std
use std.solids

in std.front {
  o := point
  fix((0, 0)) o
  base := point hint((0, -6))
  c := point hint((8, -6))
  rim := point hint((8, 5))
  apex := point hint((0, 1))
  e := point hint((4, 3))
  foot := point hint((6.68, 4.34))
  q := point hint((3.4, 3.67))
  pole := point hint((0, 5))
  o distance(-6mm, along: v) base
  base distance(8mm) c
  o distance(5mm, along: v) rim
  o distance(0mm, along: u) apex
  o distance(1mm, along: v) apex
  half := vertical line(o, base) -> horizontal line(base, c) -> vertical line(c, rim) ->
          (seat := line(rim, foot)) -> (chamfer := line(foot, q)) ->
          (dome := arc(center: o, start: q, end: pole)) -> vertical line(pole, o) -> close
  apex coincident seat
  e coincident seat
  o distance(5mm) e
  foot distance(3mm) e
  construction back := line(foot, e)
  back angle(15deg, sense: cw) chamfer
  radius(5mm) dome
}

part := solid(std.solids.Revolution(half, about: std.z))
