// The box from the origin to (30, 20, 10) with its four vertical edges rounded to radius 4, its
// eight sides drafted 5° with the pull along +z about the floor. The rounded outline is drawn in
// the top plane (x, y), the parting plane — the box's 30 × 20 rectangle, each corner rounded 4 —
// and the drafted prism over it runs to z = 10: each flat side leans 5° in as it rises, and each
// quarter cylinder becomes a quarter cone about the same axis, its radius shrinking with the
// sides, so it stays tangent to them.
unit mm
use std
use std.solids

in std.top {
  a := point
  b := point hint((30, 0))
  c := point hint((30, 20))
  d := point hint((0, 20))
  outline := horizontal line(a, b) -> round(r: 4mm) vertical line(b, c) -> round(r: 4mm)
             horizontal line(c, d) -> round(r: 4mm) vertical line(d, a) -> round(r: 4mm) close
  fix((0, 0)) a
  a distance(30) b
  b distance(20) c
}

part := solid(std.solids.Drafted(outline, from: 0mm, to: 10mm, draft: 5deg))
