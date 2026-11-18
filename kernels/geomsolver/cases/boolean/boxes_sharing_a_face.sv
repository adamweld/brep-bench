// Two cubes of side 10 side by side on the top plane, the first over the square from the origin
// to (10, 10) and the second over the square from (10, 0) to (20, 10), both from z = 0 to z = 10:
// they touch along the whole face x = 10, and the part is their union.
unit mm
use std
use std.solids
use std.solids (union)

in std.top {
  first_sq := step(u: 10mm, v: 0mm) (fs := line) -> step(v: 10mm) (fe := line) ->
              step(u: 0mm) (fn := line) -> step(v: 0mm) (fw := line) -> close
  second_sq := step(u: 20mm, v: 0mm) (ss := line) -> step(v: 10mm) (se := line) ->
               step(u: 10mm) (sn := line) -> step(v: 0mm) (sw := line) -> close
}

second := solid(std.solids.Prism(second_sq, from: 0mm, to: 10mm))
part := solid(std.solids.Prism(first_sq, from: 0mm, to: 10mm))
second union part
