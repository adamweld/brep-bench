// An L-shaped block, its six sides drafted 6° with the pull along +z about the floor: the outline
// through (0, 0), (30, 0), (30, 10), (10, 10), (10, 25) and (0, 25) drawn in the top plane (x, y),
// the parting plane, and the drafted prism over it to z = 12, each side leaning 6° from the z
// direction into the material as it rises — at the concave corner too, where the two sides meet
// along a new edge where their planes cross.
unit mm
use std
use std.solids

in std.top {
  o := point
  fix((0, 0)) o
  a := point hint((30, 0))
  b := point hint((30, 10))
  c := point hint((10, 10))
  d := point hint((10, 25))
  e := point hint((0, 25))
  outline := horizontal line(o, a) -> vertical line(a, b) -> horizontal line(b, c) ->
             vertical line(c, d) -> horizontal line(d, e) -> vertical line(e, o) -> close
  o distance(30) a
  a distance(10) b
  o distance(10, along: u) c
  o distance(25) e
}

part := solid(std.solids.Drafted(outline, from: 0mm, to: 12mm, draft: 6deg))
