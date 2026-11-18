// A box from the origin to (10, 8, 6), its 10 × 8 footprint drawn in the top plane and the prism
// over it 6 tall, hollowed to walls 1 thick with no opening: `shell(t: …) X` keeps what lies within
// 1 of everything outside the box, so a closed cavity stands inside it.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 8))
  d := point hint((0, 8))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
    horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(8) c
}

part := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 6mm))
shell(t: 1mm) part
