// A plate, the box from (0, -5, 0) to (20, 5, 4), its footprint in the top plane; a drill about the
// upright line through (10, 0), a cylinder of radius 2 up to z = 14 ended below in a cone of 118
// degrees (each side 59 degrees from the axis) whose point is (10, 0, 6), a turn of its
// half-section in the front plane; slid without turning 20 straight down along its axis, and
// everything it passes through cut from the plate.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (slides)

in std.top {
  a := point
  b := point hint((20, -5))
  c := point hint((20, 5))
  d := point hint((0, 5))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, -5)) a
  a distance(20) b
  b distance(10) c
}
part := solid(std.solids.Prism(outline, from: 0mm, to: 4mm))

in std.front {
  tip := point
  lip := point hint((12, 7.2))
  shank := point hint((12, 14))
  back := point hint((10, 14))
  fix((10, 6)) tip
  fix((10, 14)) back
  tip distance(2, along: u) lip
  half := (cone := line(tip, lip)) -> vertical line(lip, shank) ->
          horizontal line(shank, back) -> vertical (spindle := line(back, tip)) -> close
  cone angle(59deg) std.z
}
drill := solid(std.solids.Revolution(half, about: spindle))

// the push, 20 straight down over the roll
down := axis hint(dir: (0, 0, -1))
down parallel(sense: opposite) std.z
ram := frame
ram slides(20mm) down
hole := solid({ p | some t in (0deg, 360deg): p in drill at(under: ram) t })
hole cut part
