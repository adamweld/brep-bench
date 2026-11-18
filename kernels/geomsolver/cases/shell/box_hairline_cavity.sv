// A box from the origin to (10, 8, 6) shelled to walls 3.99 thick, open through its top: the walls
// at y = 0 and y = 8 come within 0.02 of each other, so the cavity is a slot 0.02 wide.
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
part.near shell(t: 3.99mm) part
