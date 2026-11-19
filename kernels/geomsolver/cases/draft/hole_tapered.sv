// The block from (-15, -15, 0) to (15, 15, 20), bored right through along the z axis by a hole of
// radius 6, the hole's wall drafted 3° with the pull along +z about the block's floor, so that the
// hole widens toward the top. The block is its footprint drawn in the top plane and the prism over
// it; the hole's circle is drawn there too, in the floor's plane, and the hole is the drafted prism
// over it through the block, its draft negated so that its wall leans out as it rises — a cone
// through the circle at the floor — cut from the block.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  a := point hint((-15, -15))
  b := point hint((15, -15))
  c := point hint((15, 15))
  d := point hint((-15, 15))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  a distance(30) b
  b distance(30) c
  o distance(15, along: u) c
  o distance(15, along: v) c
  bore := circle(center: o) hint(r: 6)
  radius(6mm) bore
}
part := solid(std.solids.Prism(footprint, from: 0mm, to: 20mm))

hole := solid(std.solids.Drafted(face(bore), from: 0mm, to: 20mm, draft: -3deg))
hole cut part
