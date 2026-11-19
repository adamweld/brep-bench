// A block, its 20 × 20 footprint drawn in the top plane (x, y) about the origin and the prism over
// it to z = 10, with a blind bore of radius 4 about the z axis whose floor's edge is beveled. The
// cutter is a turn of its half-section, drawn in the front plane (x, z) and turned a whole turn
// about the z axis: the floor at z = 4 out to radius 3, the bevel from there to radius 4 at z = 5,
// and the wall at radius 4 up past the top. Then the bevel deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the bevel runs the faces round it on along their own surfaces until
// they meet, so what is left is bounded by the surfaces of every face of the bored block but the
// bevel, read from its own faces: on the material's side of each of the block's six, and of the
// bore's wall or its floor.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  a := point
  b := point hint((10, -10))
  c := point hint((10, 10))
  d := point hint((-10, 10))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
  horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  fix((-10, -10)) a
  a distance(20) b
  b distance(20) c
}

in std.front {
  s0 := point hint((0, 4))
  s1 := point hint((3, 4))
  s2 := point hint((4, 5))
  s3 := point hint((4, 11))
  s4 := point hint((0, 11))
  horizontal (floor := line(s0, s1)) -> (bevel := line(s1, s2)) ->
  vertical (wall := line(s2, s3)) -> horizontal (mouth := line(s3, s4)) ->
  vertical (axis_side := line(s4, s0)) -> close
  fix((0, 4)) s0
  s0 distance(3) s1
  s0 distance(7) s4
  s4 distance(4) s3
  s3 distance(6) s2
}
sink := solid(std.solids.Revolution(face(floor, bevel, wall, mouth, axis_side), about: std.z))
sink cut block

block := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 10mm))

part := solid({ p |
  p in block.near.side
  p in block.far.side
  p in block.south.side
  p in block.east.side
  p in block.north.side
  p in block.west.side
  p in block.sink.wall.side or p in block.sink.floor.side
})
