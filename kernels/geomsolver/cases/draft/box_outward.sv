// The box from the origin to (20, 12, 10), its four sides drafted 5° the other way, with the pull along +z about
// the floor: its 20 × 12 footprint drawn in the top plane (x, y), the parting plane, and the
// drafted prism over it to z = 10, each side leaning 5° from the z direction out of the material as
// it rises: a negative draft.
unit mm
use std
use std.solids

in std.top {
  a := point
  b := point hint((20, 0))
  c := point hint((20, 12))
  d := point hint((0, 12))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(20) b
  b distance(12) c
}

part := solid(std.solids.Drafted(footprint, from: 0mm, to: 10mm, draft: -5deg))
