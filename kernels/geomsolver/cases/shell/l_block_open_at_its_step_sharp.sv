// An L-shaped block, its outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5)
// drawn in the top plane and extruded 20, shelled to walls 1 thick and open through its step, the
// face y = 5 its edge `ledge` sweeps, the walls' corners kept sharp: the wall of the face x = 15,
// which rises from the opening's edge, runs down to the opening's plane and is not cut by it.
//
// `std.rounds`' `shell` keeps every point within the thickness of a face kept, so below the
// opening's plane it keeps a quarter round about the foot of the wall x = 15, and it has no way to
// keep that wall sharp. So the moved walls are drawn: each edge of the cavity's outline 1 inside
// the outline's edge it lines, the edge along the opening on the opening's own line, and the prism
// over it between the caps moved in 1 is cut from the block.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  l0 := point
  l1 := point hint((20, 0))
  l2 := point hint((20, 20))
  l3 := point hint((15, 20))
  l4 := point hint((15, 5))
  l5 := point hint((0, 5))
  horizontal (e0 := line(l0, l1)) -> vertical (e1 := line(l1, l2)) -> horizontal (e2 := line(l2, l3)) ->
    vertical (e3 := line(l3, l4)) -> horizontal (ledge := line(l4, l5)) -> vertical (e5 := line(l5, l0)) -> close
  fix((0, 0)) l0
  l0 distance(20) l1
  l1 distance(20) l2
  l2 distance(5) l3
  l0 distance(5) l5

  // the walls moved in 1, each the line 1 inside the edge it lines; the opening's edge kept on
  // its own line
  k0 := point hint((1, 1))
  k1 := point hint((19, 1))
  k2 := point hint((19, 19))
  k3 := point hint((16, 19))
  k4 := point hint((16, 5))
  k5 := point hint((1, 5))
  horizontal (m0 := line(k0, k1)) -> vertical (m1 := line(k1, k2)) -> horizontal (m2 := line(k2, k3)) ->
    vertical (m3 := line(k3, k4)) -> horizontal (m4 := line(k4, k5)) -> vertical (m5 := line(k5, k0)) -> close
  m0 distance(1mm) e0
  m1 distance(1mm) e1
  m2 distance(1mm) e2
  m3 distance(1mm) e3
  k4 coincident ledge
  m5 distance(1mm) e5
}

part := solid(std.solids.Prism(face(e0, e1, e2, e3, ledge, e5), from: 0mm, to: 20mm))
cavity := solid(std.solids.Prism(face(m0, m1, m2, m3, m4, m5), from: 1mm, to: 19mm))
cavity cut part
