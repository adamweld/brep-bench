// A ball-tipped cone shrunk by 1.5: its half-section drawn in the front plane (x, z) — the base out
// to radius 6 at z = 0, the side running up toward the apex at (0, 12) until it meets, tangent,
// the arc of radius 1 about a centre on the axis that rounds the tip — turned a whole turn about
// the z axis; and the points of it at least 1.5 from its outside, `std.rounds.Grown`'s body for a
// negative reach.
unit mm
use std
use std.place (vertical)
use std.solids
use std.rounds

in std.front {
  o := point
  rim := point hint((6, 0))
  t := point hint((0.9, 10.2))
  c := point hint((0, 9.8))
  tip := point hint((0, 10.8))
  construction apex := point hint((0, 12))
  half := horizontal line(o, rim) -> (side := line(rim, t)) ->
          tangent (cap := arc(center: c, start: t, end: tip)) -> vertical line(tip, o) -> close
  fix((0, 0)) o
  o distance(6) rim
  o vertical apex
  o distance(12) apex
  apex coincident side
  o vertical c
  radius(1) cap
}

cone := solid(std.solids.Revolution(half, about: std.z))
shrunk := std.rounds.Grown(cone, d: -1.5mm)
part := solid(shrunk.body)
