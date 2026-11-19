// Two cubes of side 10: the first over the square from the origin to (10, 10) in the top plane,
// from z = 0 to z = 10; the second over the square from (5, 0) to (15, 10), from z = 10 to z = 20,
// standing on the first's top and covering half of it. The part is their union.
unit mm
use std
use std.solids
use std.solids (union)

in std.top {
  lower_sq := step(u: 10mm, v: 0mm) (ls := line) -> step(v: 10mm) (le := line) ->
              step(u: 0mm) (ln := line) -> step(v: 0mm) (lw := line) -> close
  upper_sq := step(u: 15mm, v: 0mm) (us := line) -> step(v: 10mm) (ue := line) ->
              step(u: 5mm) (un := line) -> step(v: 0mm) (uw := line) -> close
}

upper := solid(std.solids.Prism(upper_sq, from: 10mm, to: 20mm))
part := solid(std.solids.Prism(lower_sq, from: 0mm, to: 10mm))
upper union part
