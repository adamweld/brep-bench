// The upper half of a ball of radius 5 about the origin, its rim chamfered by a distance and an
// angle: 2 in from the rim on the flat face, the chamfer rising from there at 45° to the flat until
// it meets the ball. The chamfer is the cone that line sweeps about the axis.
//
// The library's chamfer takes legs, not an angle, and its flat bows beside a ball (the distances
// from the two surfaces, each over the other's leg, summing to one). So the chamfered half-section
// is drawn in the front plane (x the radius, z up) with the chamfer as one of its sides, and turned
// about the z axis: along the flat from the centre to `foot`, 2 in from the rim `e`; the chamfer at
// 45° to the flat, up to `q` on the ball; the ball's arc up to the pole; down the axis.
unit mm
use std
use std.solids

in std.front {
  o := point
  fix((0, 0)) o
  e := point hint((5, 0))
  foot := point hint((3, 0))
  q := point hint((4.7, 1.7))
  pole := point hint((0, 5))
  o distance(5mm, along: u) e
  o distance(0mm, along: v) e
  foot distance(2mm, along: u) e
  half := horizontal (flat := line(o, foot)) -> (chamfer := line(foot, q)) ->
          (dome := arc(center: o, start: q, end: pole)) -> vertical line(pole, o) -> close
  flat angle(45deg) chamfer
  radius(5mm) dome
}

part := solid(std.solids.Revolution(half, about: std.z))
