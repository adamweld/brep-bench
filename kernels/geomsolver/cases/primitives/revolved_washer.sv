// A washer: the rectangle x from 6 to 10, z from 0 to 4, drawn in the front plane (x, z) as a chain
// of runs, turned a whole turn about the z axis.
unit mm
use std
use std.solids

in std.front {
  section := step(u: 10mm, v: 0mm) (base := line) -> step(v: 4mm) (outer := line) ->
             step(u: 6mm) (top := line) -> step(v: 0mm) (bore := line) -> close
}

part := solid(std.solids.Revolution(section, about: std.z))
