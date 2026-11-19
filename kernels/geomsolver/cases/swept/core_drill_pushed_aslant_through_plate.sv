// A plate, the box from (-10, -10, 0) to (10, 10, 4), its footprint in the top plane; a core drill,
// the tube between radii 1.5 and 2.5 about the line through (0, 0, 2) along u = (sin 30, 0, cos 30)
// (z turned 30 degrees about y), running from 18 to 6 before that point: its annulus drawn in the
// plane through (0, 0, 2) square to u and carried along u. It slides 24 along u, from wholly below
// the plate to wholly above it, and everything it passes through is cut from the plate; the core
// inside it stays, cut free.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (slides)

in std.top {
  a := point
  b := point hint((10, -10))
  c := point hint((10, 10))
  d := point hint((-10, 10))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-10, -10)) a
  a distance(20) b
  b distance(20) c
}
part := solid(std.solids.Prism(outline, from: 0mm, to: 4mm))

// the drill's axis, and the plane square to it through (0, 0, 2)
lean := axis hint(dir: (0.5, 0, 0.87))
lean perpendicular std.y
std.z angle(30deg, about: std.y) lean
across := axis hint(dir: (0.87, 0, -0.5))
across perpendicular std.y
std.x angle(30deg, about: std.y) across
along := axis hint(dir: (0, 1, 0))
along parallel(sense: same) std.y
square := plane(u: across, v: along)
fix(origin == (0, 0, 2)) square
in square {
  o := point
  fix((0, 0)) o
  wall := circle(center: o) hint(r: 2.5)
  bore := circle(center: o) hint(r: 1.5)
  radius(2.5) wall
  radius(1.5) bore
}
drill := solid(std.solids.Prism(face(wall, holes: bore), from: -18mm, to: -6mm))

// the push, 24 along the drill's own axis over the roll
ram := frame
ram slides(24mm) lean
hole := solid({ p | some t in (0deg, 360deg): p in drill at(under: ram) t })
hole cut part
