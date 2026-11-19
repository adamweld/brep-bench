// A plate, the cylinder of radius 8 on the z axis from z = -6 to z = 0, with a ball of radius 5
// about the origin standing in it; the ball meets the plate's top in the circle of radius 5, a
// concave edge. The edge chamfered by a distance and an angle: 2 out from it on the plate's top,
// the chamfer rising from there at 45° to the plate back toward the ball until it meets it,
// filling the groove. The chamfer is the cone that line sweeps about the axis.
//
// The library's chamfer takes legs, not an angle, and its flat bows beside a ball (the distances
// from the two surfaces, each over the other's leg, summing to one). So the chamfered half-section
// is drawn in the front plane (x the radius, z up) with the chamfer as one of its sides, and turned
// about the z axis: across the plate's underside and up its rim; in along its top to `foot`, 2
// out from the edge `e`; the chamfer at 45° to the top, up to `q` on the ball; the ball's arc to
// the pole; down the axis.
unit mm
use std
use std.solids

in std.front {
  o := point
  fix((0, 0)) o
  base := point hint((0, -6))
  c := point hint((8, -6))
  rim := point hint((8, 0))
  e := point hint((5, 0))
  foot := point hint((7, 0))
  q := point hint((4, 3))
  pole := point hint((0, 5))
  o distance(-6mm, along: v) base
  base distance(8mm) c
  o distance(0mm, along: v) rim
  o distance(5mm, along: u) e
  o distance(0mm, along: v) e
  e distance(2mm, along: u) foot
  half := vertical line(o, base) -> horizontal line(base, c) -> vertical line(c, rim) ->
          horizontal (top := line(rim, foot)) -> (chamfer := line(foot, q)) ->
          (dome := arc(center: o, start: q, end: pole)) -> vertical line(pole, o) -> close
  top angle(45deg, sense: cw) chamfer
  radius(5mm) dome
}

part := solid(std.solids.Revolution(half, about: std.z))
