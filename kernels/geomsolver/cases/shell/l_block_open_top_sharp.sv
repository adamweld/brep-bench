// An L-shaped plate, its outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15)
// drawn in the top plane and extruded 8, shelled to walls 1 thick and open through its top, every
// pair of moved walls extended until they meet, the concave edge's too, so no round is added.
//
// `std.rounds`' `shell` keeps every point within the thickness of a face kept, which rounds the
// cavity about a concave edge, and it has no way to keep that corner sharp. So the moved walls
// are drawn: each edge of the cavity's outline 1 inside the outline's edge it lines, meeting its
// neighbours, and the prism over it from the bottom moved up 1 out through the top is cut from
// the plate.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  l0 := point
  l1 := point hint((20, 0))
  l2 := point hint((20, 5))
  l3 := point hint((5, 5))
  l4 := point hint((5, 15))
  l5 := point hint((0, 15))
  horizontal (e0 := line(l0, l1)) -> vertical (e1 := line(l1, l2)) -> horizontal (e2 := line(l2, l3)) ->
    vertical (e3 := line(l3, l4)) -> horizontal (e4 := line(l4, l5)) -> vertical (e5 := line(l5, l0)) -> close
  fix((0, 0)) l0
  l0 distance(20) l1
  l1 distance(5) l2
  l2 distance(15) l3
  l0 distance(15) l5
  l5 distance(5) l4

  // the walls moved in 1, each the line 1 inside the edge it lines
  k0 := point hint((1, 1))
  k1 := point hint((19, 1))
  k2 := point hint((19, 4))
  k3 := point hint((4, 4))
  k4 := point hint((4, 14))
  k5 := point hint((1, 14))
  horizontal (m0 := line(k0, k1)) -> vertical (m1 := line(k1, k2)) -> horizontal (m2 := line(k2, k3)) ->
    vertical (m3 := line(k3, k4)) -> horizontal (m4 := line(k4, k5)) -> vertical (m5 := line(k5, k0)) -> close
  m0 distance(1mm) e0
  m1 distance(1mm) e1
  m2 distance(1mm) e2
  m3 distance(1mm) e3
  m4 distance(1mm) e4
  m5 distance(1mm) e5
}

part := solid(std.solids.Prism(face(e0, e1, e2, e3, e4, e5), from: 0mm, to: 8mm))
cavity := solid(std.solids.Prism(face(m0, m1, m2, m3, m4, m5), from: 1mm, to: 8mm))
cavity cut part
