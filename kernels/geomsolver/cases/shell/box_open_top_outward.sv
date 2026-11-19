// A box from the origin to (10, 8, 6) shelled outward to walls 1 thick, open at its top: the box's
// faces but the top are the inside of the wall, and the wall is every point outside the box within
// 1 of them, up to the plane of the top.
//
// `std.rounds`' `shell` hollows a body inward only, so the outward shell is said with the words it
// is made of: the box grown by 1 (`std.rounds.Grown`, every point within 1 of it, its edges and
// corners rounded by the ball), less what stands above the top's plane (a slab standing on it,
// wider than the wall), less the box.
unit mm
use std
use std.solids
use std.solids (cut)
use std.rounds

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 8))
  d := point hint((0, 8))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
    horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(8) c
  // the slab's outline, a circle about the box's middle reaching past the wall
  m := point hint((5, 4))
  m midpoint line(a, c)
  reach := circle(center: m) hint(r: 20)
  radius(20mm) reach
}

box := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 6mm))
grown := std.rounds.Grown(box, d: 1mm)
above := solid(std.solids.Prism(face(reach), from: 6mm, to: 8mm))
part := solid(grown.body)
above cut part
box cut part
