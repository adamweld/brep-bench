// An L-shaped plate: its outline drawn in the top plane (x, y) as a chain of runs from the origin,
// each run one number — out 20 along x, up 5, back to x = 5, up to y = 15, back to the y axis and
// down to the origin — and the prism over it from z = 0 to z = 8.
unit mm
use std
use std.solids

in std.top {
  outline := step(u: 20mm, v: 0mm) (foot := line) -> step(v: 5mm) (toe := line) ->
             step(u: 5mm) (ledge := line) -> step(v: 15mm) (riser := line) ->
             step(u: 0mm) (head := line) -> step(v: 0mm) (back := line) -> close
}

part := solid(std.solids.Prism(outline, from: 0mm, to: 8mm))
