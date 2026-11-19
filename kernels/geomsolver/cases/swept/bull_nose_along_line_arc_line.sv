// A block, the box from (-10, -10, -10) to (40, 40, 0), its top in the top plane; an upright
// bull-nose mill, its axis on the vertical line through (0, 0): radius 2 from its flat bottom at
// z = -1.5 up to z = 5, the corner of its bottom rounded to 0.5, a turn of its half-section in the
// front plane. Its axis moves, staying upright, along a path of three pieces in the top face,
// tangent where they meet: it slides along the line from (0, 0) to (20, 0), turns a quarter turn
// about the upright line through (20, 10) to (30, 10), and slides on along the line to (30, 30);
// everything the mill passes through in each piece is cut from the block.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (slides, turns)

in std.top {
  a := point
  b := point hint((40, -10))
  c := point hint((40, 40))
  d := point hint((-10, 40))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-10, -10)) a
  a distance(50) b
  b distance(50) c

  // the path, a line, an arc and a line, each tangent to the next
  p0 := point
  p1 := point hint((20, 0))
  k := point
  p2 := point hint((30, 10))
  p3 := point hint((30, 30))
  fix((0, 0)) p0
  fix((20, 10)) k
  p0 distance(20) p1
  p2 distance(20) p3
  route := horizontal (run_in := line(p0, p1)) -> tangent
           (bend := arc(center: k, start: p1, end: p2)) -> tangent
           vertical (run_out := line(p2, p3))
  radius(10) bend
}
part := solid(std.solids.Prism(outline, from: -10mm, to: 0mm))

in std.front {
  e0 := point
  q := point hint((2, -1.5))
  e2 := point hint((2, 5))
  e3 := point hint((0, 5))
  fix((0, -1.5)) e0
  e0 distance(2) q
  fix((0, 5)) e3
  // the side meets the flat bottom at q, the corner rounded to 0.5
  half := horizontal line(e0, q) -> round(r: 0.5mm) vertical line(q, e2) ->
          horizontal line(e2, e3) -> vertical (spindle := line(e3, e0)) -> close
}
mill := solid(std.solids.Revolution(half, about: spindle))

// the motion in its three pieces, each carrying the mill on from where the last left it: a slide
// along the first line, a quarter turn about the upright line through the arc's centre, a slide
// along the last line
k_above := point
fix((20, 10, 1)) k_above
pivot := line(k, k_above)
first := frame
first slides(20mm) run_in
turn := frame
turn turns(1) pivot
last := frame
last slides(20mm) run_out
at_bend := mill at(under: first) 360deg
past_bend := at_bend at(under: turn) 90deg
straight_in := solid({ p | some t in (0deg, 360deg): p in mill at(under: first) t })
round_bend := solid({ p | some t in (0deg, 90deg): p in at_bend at(under: turn) t })
straight_out := solid({ p | some t in (0deg, 360deg): p in past_bend at(under: last) t })
straight_in cut part
round_bend cut part
straight_out cut part
