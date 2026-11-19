// A shelf bracket: its L outline drawn in the front plane (x, z) as a closed chain of runs, the
// inside corner between the post and the arm rounded to radius 8 by a round at that joint, and
// the prism over it 10 thick, y from −5 to 5.
unit mm
use std
use std.solids

H := 120mm        // the post's height
reach := 100mm    // the arm's reach
w := 20mm         // the post's width and the arm's depth
r := 8mm          // the inside corner's round
t := 10mm

in std.front {
  a := point
  fix((0mm, 0mm)) a
  corner := point hint((20, 100))    // the inside corner, which the round takes
  outline := step(u: w) (foot := line(a)) -> (inside := line(p2: corner)) -> round(r: r)
             step(u: reach) (under := line(corner)) -> step(dv: w) (tip := line) ->
             step(u: 0mm) (top := line) -> (back := line(p2: a)) -> close
  foot.p2 level(right) corner
  a distance(H - w, along: up) corner
}
part := solid(std.solids.Prism(outline, from: -t / 2, to: t / 2))
