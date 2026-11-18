// A square plate from (-20, -20, 0) to (20, 20, 5), and on it a boss of radius 8 about the z axis
// standing 12 up from its top, its side drafted 4° with the pull along +z about the plate's top,
// where it stands. The plate is its footprint drawn in the top plane and the prism over it; the
// boss's circle is drawn on a plane lying on the plate's top face (`plate.near`), centred on the
// z axis, and the boss is the drafted prism over it to 12 up — a cone leaning 4° in as it rises —
// united with the plate.
unit mm
use std
use std.solids
use std.solids (union)

in std.top {
  o := point
  fix((0, 0)) o
  a := point hint((-20, -20))
  b := point hint((20, -20))
  c := point hint((20, 20))
  d := point hint((-20, 20))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  a distance(40) b
  b distance(40) c
  o distance(20, along: u) c
  o distance(20, along: v) c
}
plate := solid(std.solids.Prism(footprint, from: 0mm, to: 5mm))

deck := plane(u: hint(dir: (1, 0, 0)), v: hint(dir: (0, 1, 0))) hint(origin: (0, 0, 5))
deck.origin in boundary(plate.near.side)
deck tangent(at: deck.origin) boundary(plate.near.side)
deck.origin coincident std.z
deck.u perpendicular std.y
deck.v perpendicular std.x
in deck {
  rim := circle(center: deck.origin) hint(r: 8)
  radius(8mm) rim
}
boss := solid(std.solids.Drafted(face(rim), from: 0mm, to: 12mm, draft: 4deg))

part := solid(plate)
boss union part
