// The box from the origin to (20, 12, 10), its four sides drafted 5° with the pull along +z about
// the neutral plane z = 5: each side turns about the line where it crosses z = 5, moving out below
// it and in above it. The box's 20 × 12 rectangle is drawn on the plane held at z = 5. Solvent's
// draft leans the sides away from the face's own plane on both sides (a parting line), so the one
// lean through the neutral plane is two drafts joined there: above it the drafted prism to 5 up,
// leaning 5° in as it rises, and below it the drafted prism to 5 down with the draft negated,
// leaning 5° out as it falls.
unit mm
use std
use std.solids
use std.solids (union)

neutral := plane
fix(origin == (0mm, 0mm, 5mm)) neutral
fix(dir == (1, 0, 0)) neutral.u
fix(dir == (0, 1, 0)) neutral.v

in neutral {
  a := point
  b := point hint((20, 0))
  c := point hint((20, 12))
  d := point hint((0, 12))
  section := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(20) b
  b distance(12) c
}

part := solid(std.solids.Drafted(section, from: 0mm, to: 5mm, draft: 5deg))
below := solid(std.solids.Drafted(section, from: -5mm, to: 0mm, draft: -5deg))
below union part
