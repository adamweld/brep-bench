// A hoof cut from a cylinder: the rod of radius 5 on the z axis from z = -1 to z = 10, kept above
// the top plane and below the plane through the y axis that rises at 30° toward +x.
unit mm
use std
use std.sets

low := point
fix((0, 0, -1)) low
high := point
fix((0, 0, 10)) high
spine := line(low, high)
rod := std.sets.Rod(spine, r: 5mm)

in std.front {
  o := point
  fix((0, 0)) o
  e := point
  fix((1, 0)) e
  t := point hint((0.87, 0.5))
  run := line(o, e)
  rise := line(o, t)
  length(1mm) rise
  run angle(30deg) rise
}
across := point
fix((0, 1, 0)) across
// the slanting plane over the rise and the y axis; its normal, rise × y, leans up
y_line := line(o, across)
slant := plane(u: rise, v: y_line)
part := solid({ p | p in rod; p distance(>= 0mm, along: n) std.top; p distance(<= 0mm, along: n) slant })
