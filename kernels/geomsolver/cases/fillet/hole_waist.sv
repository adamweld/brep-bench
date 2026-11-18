// A plate from (−10, −10, 0) to (10, 10, 2) with a hole of radius 3 on the z axis, a cutter
// through it; both rims of the hole — every edge where the drill's face meets the plate's —
// rounded to radius 1, half the plate's thickness, so the two rounds meet at the hole's waist.
unit mm
use std
use std.solids
use std.solids (cut)
use std.rounds (round)

in std.top {
  footprint := step(u: 10mm, v: -10mm) (south := line) -> step(v: 10mm) (east := line) ->
               step(u: -10mm) (north := line) -> step(v: -10mm) (west := line) -> close
  o := point
  fix((0, 0)) o
  hole := circle(center: o) hint(r: 3)
  radius(3mm) hole
}

drill := solid(std.solids.Through(face(hole)))
part := solid(std.solids.Prism(footprint, from: 0mm, to: 2mm))
drill cut part
part.drill round(r: 1mm) part
