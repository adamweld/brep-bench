// A curved block, the ring sector between radii 8 and 12 about the z axis, z within 1, polar angle
// from 0 to 60 degrees: its section in the front plane (x, z) turned 60 degrees about the z axis.
// The tool, a ball of radius 0.8 about (10, 0, 0) in union with a stem, the rod of radius 0.4
// along the segment 3 long from the ball's centre leaning 20 degrees from upright toward +y. It is
// turned rigidly about the z axis from -140 to +30 degrees, and everything it passes through cut
// from the block.
unit mm
use std
use std.sets
use std.solids
use std.solids (cut, union)
use std.motion (turns)

in std.front {
  a := point
  b := point hint((12, -1))
  c := point hint((12, 1))
  d := point hint((8, 1))
  meridian := horizontal line(a, b) -> vertical line(b, c) ->
              horizontal line(c, d) -> vertical line(d, a) -> close
  fix((8, -1)) a
  a distance(4) b
  b distance(2) c
}
part := solid(std.solids.Revolution(meridian, about: std.z, to: 60deg))

centre := point
fix((10, 0, 0)) centre
top := point hint((10, 1.03, 2.82))
stem_axis := line(centre, top)
length(3) stem_axis
stem_axis perpendicular std.x
stem_axis angle(20deg, about: std.x) std.z
ball := solid(std.sets.Ball(centre, r: 0.8mm))
stem := solid(std.sets.Rod(stem_axis, r: 0.4mm))
tool := solid(ball)
stem union tool

swing := frame
swing turns(1) std.z
notch := solid({ p | some t in (-140deg, 30deg): p in tool at(under: swing) t })
notch cut part
