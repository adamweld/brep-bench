// A plate from (−20, −20, 0) to (20, 20, 4), and a boss on it, the cylinder of radius 5 on the z
// axis from z = 4 to z = 8, in union. Both circles of the boss rounded to radius 2: every edge
// among the boss's own faces (its top's rim, convex) and every edge where the boss meets the plate
// (its root, concave, filled). The two rounds meet tangent at z = 6, and the side vanishes.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds (round)

in std.top {
  footprint := step(u: 20mm, v: -20mm) (south := line) -> step(v: 20mm) (east := line) ->
               step(u: -20mm) (north := line) -> step(v: -20mm) (west := line) -> close
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 5)
  radius(5mm) rim
}

plate := solid(std.solids.Prism(footprint, from: 0mm, to: 4mm))
boss := solid(std.solids.Prism(face(rim), from: 4mm, to: 8mm))
part := solid(plate)
boss union part
round(r: 2mm) part.boss
part.boss round(r: 2mm) part.plate
