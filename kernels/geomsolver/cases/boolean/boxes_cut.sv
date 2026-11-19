// Two blocks 10 high standing on the top plane: the first over the square from (0, 0) to
// (10, 10), the second over the square from (5, 5) to (15, 15), each footprint a chain of runs.
// The part is the first, less the second cut from it.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  first_sq := step(u: 10mm, v: 0mm) (fs := line) -> step(v: 10mm) (fe := line) ->
              step(u: 0mm) (fn := line) -> step(v: 0mm) (fw := line) -> close
  second_sq := step(u: 15mm, v: 5mm) (ss := line) -> step(v: 15mm) (se := line) ->
               step(u: 5mm) (sn := line) -> step(v: 5mm) (sw := line) -> close
}

second := solid(std.solids.Prism(second_sq, from: 0mm, to: 10mm))
part := solid(std.solids.Prism(first_sq, from: 0mm, to: 10mm))
second cut part
