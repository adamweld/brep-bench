// An L-block: the foot, the box from the origin to (20, 10, 6), and the upright, the box from the
// origin to (6, 10, 20), in union, each section drawn in the front plane (x, z) as a chain of runs
// and its prism taken from y = 0 to y = 10 (the front's normal is −y, so y = 0 is each `near`).
// The rounds are made one after another, as the case says: first the concave edge where the
// foot's top (`ft`) meets the upright's face x = 6 (`ue`), rounded to radius 2 and filled (`cove`);
// then, on a solid of the body that round leaves, the chain of the end face y = 0 against the
// upright's face, the cove's round and the foot's top, rounded to radius 2.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds (round)

in std.front {
  foot_sec := step(u: 20mm, v: 0mm) (fb := line) -> step(v: 6mm) (fe := line) ->
              step(u: 0mm) (ft := line) -> step(v: 0mm) (fw := line) -> close
  upright_sec := step(u: 6mm, v: 0mm) (ub := line) -> step(v: 20mm) (ue := line) ->
                 step(u: 0mm) (ut := line) -> step(v: 0mm) (uw := line) -> close
}

foot := solid(std.solids.Prism(foot_sec, from: -10mm, to: 0mm))
upright := solid(std.solids.Prism(upright_sec, from: -10mm, to: 0mm))
body := solid(foot)
upright union body

// 1. the concave edge
cove := body.foot.ft round(r: 2mm) body.upright.ue

// 2. the end face's chain, over the body as the cove leaves it: the foot's and the upright's ends
// against their own faces, and the cove's end (its face square to y that meets the foot's end)
// against the cove's round
part := solid(body)
part.body.foot.near round(r: 2mm) part.body.foot.ft
part.body.upright.near round(r: 2mm) part.body.upright.ue
repeat c in part.body.cove {
  when c perpendicular std.y and c meets part.body.foot.near {
    c round(r: 2mm) part.body.cove
  }
}
