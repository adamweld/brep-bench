// A box from the origin to (10, 8, 6) shelled to walls 1 thick, open through every face but its
// bottom, `part.far`: only the bottom keeps a wall, a slab 1 thick.
//
// `std.rounds`' `A shell(t: …) B` takes one face path as its opening, so with five openings its
// own words are written out here: the cavity is the box's inside less the wall within 1 of the one
// face kept, and it is cut from the box.
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
}

part := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 6mm))
within := { c | c in part }
cavity := solid(within)
repeat w in part {
  when w == part.far {
    wall := { p | p distance(<= 1mm) w }
    walled := solid(wall)
    cavity and { p | p not in walled }
  }
}
cavity cut part
