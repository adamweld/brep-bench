// A box from the origin to (10, 8, 6) shelled outward to walls 1 thick, open at its top, its edges
// kept sharp: each face but the top moved 1 outward, the moved faces extended until they meet and
// cut off by the top's plane, less the box.
//
// `std.rounds`' `shell` hollows a body inward only, and a body grown (`std.rounds.Grown`) rounds
// its edges, so the moved faces are drawn: the footprint's four sides each 1 outside the box's,
// meeting square, and the prism over them from 1 below the bottom up to the top's plane, the box
// cut from it.
unit mm
use std
use std.solids
use std.solids (cut)

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

  // the outline moved out 1 from each side
  oa := point hint((-1, -1))
  ob := point hint((11, -1))
  oc := point hint((11, 9))
  od := point hint((-1, 9))
  horizontal (os := line(oa, ob)) -> vertical (oe := line(ob, oc)) ->
    horizontal (on := line(oc, od)) -> vertical (ow := line(od, oa)) -> close
  oa distance(1mm, along: u) a
  oa distance(1mm, along: v) a
  c distance(1mm, along: u) oc
  c distance(1mm, along: v) oc
}

box := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 6mm))
part := solid(std.solids.Prism(face(os, oe, on, ow), from: -1mm, to: 6mm))
box cut part
