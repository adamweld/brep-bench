// A square bar mitered round a sharp corner. The path is drawn in the top plane: 40 along +x from
// the origin, then 40 on, turned 120° to the left. Each run is its 10 × 10 section, drawn square
// to the run where the bar ends (its `v` along z), slid straight along the run past the corner;
// at the corner each is cut by the miter, the upright plane through the corner square to the
// bisector of the turn, and the two runs are united there.
unit mm
use std
use std.place (horizontal)
use std.shapes
use std.solids
use std.solids (cut, union)
use std.stations

in std.top {
  a := point
  fix((0, 0)) a
  b := point hint((40, 0))
  c := point hint((20, 34.64))
  horizontal (first := line(a, b))
  second := line(b, c)
  length(40mm) first
  length(40mm) second
  first angle(120deg) second
  // the miter's normal: the bisector of the turn, as far from the first run's way as from the
  // second's
  k := point hint((45, 9))
  bisector := line(b, k)
  length(10mm) bisector
  first angle(bisector, second) bisector
}

// the bar's ends: the section square to each run where it ends
start := std.stations.OnLine(first, up: std.z, s: 0)
finish := std.stations.OnLine(second, up: std.z, s: 1)
sq1 := std.shapes.CenteredRectangle(start.n.origin, w: 10mm, h: 10mm) in start.n
sq2 := std.shapes.CenteredRectangle(finish.n.origin, w: 10mm, h: 10mm) in finish.n

// each run's section slid along it, past the corner (the stations' normals face along the runs):
// 50 reaches beyond the miter's outer end
run1 := solid(std.solids.Prism(face(sq1.loop), from: 0mm, to: 50mm))
run2 := solid(std.solids.Prism(face(sq2.loop), from: -50mm, to: 0mm))

// the miter: the upright plane through the corner square to the bisector, and a block on each
// side of it, reaching past the bar, to cut each run there
miter := plane(u: hint(dir: (-0.87, 0.5, 0)), v: hint(dir: (0, 0, 1)))
miter.origin coincident b
miter.v parallel std.z
miter.u perpendicular bisector
miter.u perpendicular std.z
cover := std.shapes.CenteredRectangle(miter.origin, w: 60mm, h: 60mm) in miter
beyond_first := solid(std.solids.Prism(face(cover.loop), from: 0mm, to: 60mm))
beyond_second := solid(std.solids.Prism(face(cover.loop), from: -60mm, to: 0mm))

leg1 := solid(run1)
beyond_first cut leg1
leg2 := solid(run2)
beyond_second cut leg2
part := solid(leg1)
leg2 union part
