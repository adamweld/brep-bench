// An L-shaped plate, its outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15)
// drawn in the top plane and extruded 8, shelled to walls 1 thick and open through its top,
// `part.near`. The shell keeps every point within 1 of a face kept, so at the one concave edge,
// x = 5, y = 5, the cavity is rounded by a quarter circle of radius 1 about it.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.top {
  l0 := point
  l1 := point hint((20, 0))
  l2 := point hint((20, 5))
  l3 := point hint((5, 5))
  l4 := point hint((5, 15))
  l5 := point hint((0, 15))
  outline := horizontal line(l0, l1) -> vertical line(l1, l2) -> horizontal line(l2, l3) ->
             vertical line(l3, l4) -> horizontal line(l4, l5) -> vertical line(l5, l0) -> close
  fix((0, 0)) l0
  l0 distance(20) l1
  l1 distance(5) l2
  l2 distance(15) l3
  l0 distance(15) l5
  l5 distance(5) l4
}

part := solid(std.solids.Prism(outline, from: 0mm, to: 8mm))
part.near shell(t: 1mm) part
