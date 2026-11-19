// A block from the origin to (10, 4, 6), its footprint drawn in the top plane; the two edges of
// its top (`near`) that run along x — where it meets the sides swept from `south` (y = 0) and
// `north` (y = 4) — each rounded to radius 2, half the top's width: the top vanishes into a
// half-cylinder.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  footprint := step(u: 10mm, v: 0mm) (south := line) -> step(v: 4mm) (east := line) ->
               step(u: 0mm) (north := line) -> step(v: 0mm) (west := line) -> close
}

part := solid(std.solids.Prism(footprint, from: 0mm, to: 6mm))
repeat side in [part.south, part.north] {
  part.near round(r: 2mm) side
}
