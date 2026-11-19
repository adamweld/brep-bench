// A plate from the origin to (20, 10, 2), its footprint drawn in the top plane; the two edges of
// its face y = 10 (swept from `north`) that run along x — where it meets the top (`near`) and the
// bottom (`far`) — each rounded to radius 0.9999, leaving a strip 0.0002 tall between them.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  footprint := step(u: 20mm, v: 0mm) (south := line) -> step(v: 10mm) (east := line) ->
               step(u: 0mm) (north := line) -> step(v: 0mm) (west := line) -> close
}

part := solid(std.solids.Prism(footprint, from: 0mm, to: 2mm))
repeat cap in [part.near, part.far] {
  part.north round(r: 0.9999mm) cap
}
