// The block from the origin to (20, 20, 10), bored right through along z by a hole of radius 3
// about x = 15.5, y = 10, 1.5 from the side x = 20, shelled to walls 1 thick and open through its
// top, `part.near`: the tube of wall round the bore runs into the wall at x = 19 and merges with it.
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

  o := point hint((15.5, 10))
  a distance(15.5mm, along: u) o
  a distance(10mm, along: v) o
  hole := circle(center: o) hint(r: 3)
  radius(3) hole
}

part := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 10mm))
bore := solid(std.solids.Through(face(hole)))
bore cut part
part.near shell(t: 1mm) part
