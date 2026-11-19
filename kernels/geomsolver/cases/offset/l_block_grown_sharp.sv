// An L-shaped plate grown by 1 with its edges kept sharp. Solvent's grown body (`std.rounds.Grown`,
// the points within 1) rounds every convex edge, and it has no offset that extends the moved faces
// until they meet, so the moved outline is drawn instead: in the top plane (x, y), beside the
// plate's outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), each of its edges
// moved 1 outward, stated as the edge 1 from the plate's edge it moves (the runs keep their
// directions, and meet where the moved lines cross); the prism over it runs from z = −1 to z = 9,
// the plate's caps at 0 and 8 each moved 1 out.
unit mm
use std
use std.solids

grow := 1mm
thick := 8mm

in std.top {
  a := point
  b := point hint((20, 0))
  c := point hint((20, 5))
  d := point hint((5, 5))
  e := point hint((5, 15))
  f := point hint((0, 15))
  horizontal (ab := line(a, b)) -> vertical (bc := line(b, c)) -> horizontal (cd := line(c, d)) ->
  vertical (de := line(d, e)) -> horizontal (ef := line(e, f)) -> vertical (fa := line(f, a)) -> close
  fix((0, 0)) a
  a distance(20) b
  b distance(5) c
  c distance(15) d
  f distance(15) a

  // the same outline, each edge moved `grow` outward
  a1 := point hint((-1, -1))
  b1 := point hint((21, -1))
  c1 := point hint((21, 6))
  d1 := point hint((6, 6))
  e1 := point hint((6, 16))
  f1 := point hint((-1, 16))
  moved := horizontal (ab1 := line(a1, b1)) ->
           vertical (bc1 := line(b1, c1)) ->
           horizontal (cd1 := line(c1, d1)) -> vertical (de1 := line(d1, e1)) ->
           horizontal (ef1 := line(e1, f1)) -> vertical (fa1 := line(f1, a1)) -> close
  ab distance(grow) ab1
  bc distance(grow) bc1
  cd distance(grow) cd1
  de distance(grow) de1
  ef distance(grow) ef1
  fa distance(grow) fa1
}

part := solid(std.solids.Prism(moved, from: -grow, to: thick + grow))
