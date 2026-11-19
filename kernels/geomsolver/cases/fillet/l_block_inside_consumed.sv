// An L-block: its section drawn in the front plane (x, z) as a chain of runs from the origin —
// out 10 along x, up 3, back to x = 3 (`ledge`), up to z = 10 (`riser`), back to the z axis and
// down — and the prism over it from y = 0 to y = 10 (the front's normal is −y). The concave edge
// where the ledge's face meets the riser's is rounded to radius 7, filled: the round consumes both.
unit mm
use std
use std.solids
use std.rounds (round)

in std.front {
  section := step(u: 10mm, v: 0mm) (foot := line) -> step(v: 3mm) (toe := line) ->
             step(u: 3mm) (ledge := line) -> step(v: 10mm) (riser := line) ->
             step(u: 0mm) (head := line) -> step(v: 0mm) (back := line) -> close
}

part := solid(std.solids.Prism(section, from: -10mm, to: 0mm))
part.ledge round(r: 7mm) part.riser
