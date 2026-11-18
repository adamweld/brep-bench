// The octahedron |x| + |y| + |z| ≤ 10, as the join of a square and its axis: the points on a
// segment from some point of the square with corners (±10, 0, 0) and (0, ±10, 0), drawn in the top
// plane, to some point of the segment from (0, 0, −10) to (0, 0, 10). Every edge among its faces
// rounded to radius 2; four rounds meet at each corner.
unit mm
use std
use std.sets
use std.rounds (round)
use std.place (horizontal, vertical)

in std.top {
  o := point
  fix((0, 0)) o
  e := point
  fix((10, 0)) e
  n := point hint((0, 10))
  w := point hint((-10, 0))
  s := point hint((0, -10))
  equator := line(e, n) -> line(n, w) -> line(w, s) -> line(s, e) -> close
  o distance(10mm) n
  o distance(10mm) w
  o distance(10mm) s
  o vertical n
  o horizontal w
  o vertical s
  square := face(equator)
}
bottom := point
fix((0, 0, -10)) bottom
top := point
fix((0, 0, 10)) top

part := solid({ p | some q in square, r in std.sets.Segment(bottom, top): p in std.sets.Segment(q, r) })
round(r: 2mm) part
