// A pyramid on the rectangle from (−10, −6) to (10, 6) in the top plane, its apex 12 over the
// rectangle's middle: the join of the apex and the base, the points on a segment from the apex to
// some point of the base. Every edge among its faces rounded to radius 3/2.
unit mm
use std
use std.sets
use std.rounds (round)

in std.top {
  m := point
  fix((0, 0)) m
  s0 := point hint((10, -6))
  s1 := point hint((10, 6))
  s2 := point hint((-10, 6))
  s3 := point hint((-10, -6))
  (e0 := line(s0, s1)) -> (e1 := line(s1, s2)) -> (e2 := line(s2, s3)) -> (e3 := line(s3, s0)) -> close
  vertical e0
  horizontal e1
  vertical e2
  horizontal e3
  e0 distance(20mm) e2
  e1 distance(12mm) e3
  m distance(10mm) e0
  m distance(6mm) e1
  base := face(e0, e1, e2, e3)
}
apex := point hint((0, 0, 12))
m distance(12mm, along: std.z) apex
m distance(0mm, along: std.x) apex
m distance(0mm, along: std.y) apex

part := solid({ p | some q in base: p in std.sets.Segment(apex, q) })
round(r: 1.5mm) part
