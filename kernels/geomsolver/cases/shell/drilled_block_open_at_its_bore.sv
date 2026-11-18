// The block from the origin to (20, 20, 10), bored right through along z by a hole of radius 3
// about x = 10, y = 10, shelled to walls 1 thick and open through the bore's own face,
// `part.bore.hole`, the cylinder its circle sweeps: the hollow box's cavity opens into the hole.
unit mm
use std
use std.solids
use std.solids (cut)
use std.rounds (shell)

in std.top {
  a := point
  b := point hint((20, 0))
  c := point hint((20, 20))
  d := point hint((0, 20))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
    horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  fix((0, 0)) a
  a distance(20) b
  b distance(20) c

  o := point hint((10, 10))
  a distance(10mm, along: u) o
  a distance(10mm, along: v) o
  hole := circle(center: o) hint(r: 3)
  radius(3) hole
}

part := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 10mm))
bore := solid(std.solids.Through(face(hole)))
bore cut part
part.bore.hole shell(t: 1mm) part
