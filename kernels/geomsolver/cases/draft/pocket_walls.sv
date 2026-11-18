// The block from the origin to (40, 30, 15), a rectangular pocket cut into its top from (8, 8) to
// (32, 22), its floor at z = 5, and the pocket's four walls drafted 5° with the pull along +z about
// the floor, so the pocket widens toward its opening. The block is its footprint drawn in the top
// plane and the prism over it; the pocket's rectangle is drawn on the plane held at z = 5, where
// its floor lies, and the pocket is the drafted prism over it to the block's top, its draft
// negated so that its sides lean out as they rise, cut from the block.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  b := point hint((40, 0))
  c := point hint((40, 30))
  d := point hint((0, 30))
  footprint := horizontal line(o, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, o) -> close
  o distance(40) b
  b distance(30) c
}
part := solid(std.solids.Prism(footprint, from: 0mm, to: 15mm))

floor := plane
fix(origin == (0mm, 0mm, 5mm)) floor
fix(dir == (1, 0, 0)) floor.u
fix(dir == (0, 1, 0)) floor.v

in floor {
  p0 := point hint((8, 8))
  p1 := point hint((32, 8))
  p2 := point hint((32, 22))
  p3 := point hint((8, 22))
  mouth := horizontal line(p0, p1) -> vertical line(p1, p2) ->
           horizontal line(p2, p3) -> vertical line(p3, p0) -> close
  fix((8, 8)) p0
  p0 distance(24) p1
  p1 distance(14) p2
}
pocket := solid(std.solids.Drafted(mouth, from: 0mm, to: 10mm, draft: -5deg))
pocket cut part
