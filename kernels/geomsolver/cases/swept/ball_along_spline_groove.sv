// A block, the box from (-6, -6, -8) to (36, 26, 0), its top in the top plane; a ball of radius 3
// whose centre rides a frame along the cubic Bezier through the control points (0, 0), (10, 0),
// (20, 20) and (30, 20) of the top face, and everything it passes through cut from the block.
unit mm
use std
use std.sets
use std.solids
use std.solids (cut)
use std.stations

in std.top {
  a := point
  b := point hint((36, -6))
  c := point hint((36, 26))
  d := point hint((-6, 26))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-6, -6)) a
  a distance(42) b
  b distance(32) c

  // the path, a spline over its four control points
  k0 := point
  k1 := point hint((10, 0))
  k2 := point hint((20, 20))
  k3 := point hint((30, 20))
  fix((0, 0)) k0
  fix((10, 0)) k1
  fix((20, 20)) k2
  fix((30, 20)) k3
  route := spline(k0, k1, k2, k3)
}
part := solid(std.solids.Prism(outline, from: -8mm, to: 0mm))

// the ball, about the path's start; the frame that carries it along the whole spline
ball := solid(std.sets.Ball(std.origin, r: 3mm))
along := std.stations.Along(route, up: std.z)
groove := solid({ p | some t in (0rad, 1rad): p in ball at(under: along.f) t })
groove cut part
