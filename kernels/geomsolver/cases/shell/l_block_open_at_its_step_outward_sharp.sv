// An L-shaped block, its outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5)
// drawn in the top plane and extruded 20, shelled outward to walls 1 thick, open through its step,
// the face y = 5 its edge `ledge` sweeps, the edges kept sharp: each face but the step moved 1
// outward, the moved faces extended until they meet, and cut off by the step's plane only where
// the step is, so the tall leg beyond it keeps its walls.
//
// `std.rounds`' `shell` hollows a body inward only, and a body grown (`std.rounds.Grown`) rounds
// its edges, so the moved faces are drawn: each edge of the outline moved 1 out from the edge it
// lines, the step's on the step's own line, and the prism over that from 1 below the bottom to 1
// above the top, the block cut from it.
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

  // the faces moved out 1, each the line 1 outside the edge it lines; the step's kept on its line
  k0 := point hint((-1, -1))
  k1 := point hint((21, -1))
  k2 := point hint((21, 21))
  k3 := point hint((14, 21))
  k4 := point hint((14, 5))
  k5 := point hint((-1, 5))
  horizontal (m0 := line(k0, k1)) -> vertical (m1 := line(k1, k2)) -> horizontal (m2 := line(k2, k3)) ->
    vertical (m3 := line(k3, k4)) -> horizontal (m4 := line(k4, k5)) -> vertical (m5 := line(k5, k0)) -> close
  m0 distance(1mm) e0
  m1 distance(1mm) e1
  m2 distance(1mm) e2
  m3 distance(1mm) e3
  k4 coincident ledge
  m5 distance(1mm) e5
}

block := solid(std.solids.Prism(face(e0, e1, e2, e3, ledge, e5), from: 0mm, to: 20mm))
part := solid(std.solids.Prism(face(m0, m1, m2, m3, m4, m5), from: -1mm, to: 21mm))
block cut part
