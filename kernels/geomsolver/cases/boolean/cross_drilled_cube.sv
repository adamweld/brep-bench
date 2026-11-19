// A cube from the origin to (30, 30, 30) drilled through on a grid three ways, fifteen holes of
// radius 2.5 each cut from it in turn, each a cutter through the cube along its circle's plane's
// normal: three circles in the side plane (y, z), at y = 7.5, 15 and 22.5 and z = 15, drilled
// along x; three in the front plane (x, z), at x = 7.5, 15 and 22.5 and z = 15, drilled along y;
// and nine in the top plane, at x and y each 7.5, 15 and 22.5, drilled along z.
unit mm
use std
use std.solids
use std.solids (cut)
use std.place (coords)

side := 30mm
r := 2.5mm
grid := [7.5mm, 15mm, 22.5mm]

in std.top {
  footprint := step(u: side, v: 0mm) (south := line) -> step(v: side) (east := line) ->
               step(u: 0mm) (north := line) -> step(v: 0mm) (west := line) -> close
}
part := solid(std.solids.Prism(footprint, from: 0mm, to: side))

repeat a in grid {
  cx := point hint((a, 15)) in std.side
  cx coords(du: a, dv: side / 2) std.side
  along_x := circle(center: cx) hint(r: 2.5) in std.side
  radius(r) along_x
  drill_x := solid(std.solids.Through(face(along_x)))
  drill_x cut part

  cy := point hint((a, 15)) in std.front
  cy coords(du: a, dv: side / 2) std.front
  along_y := circle(center: cy) hint(r: 2.5) in std.front
  radius(r) along_y
  drill_y := solid(std.solids.Through(face(along_y)))
  drill_y cut part

  repeat b in grid {
    cz := point hint((a, b)) in std.top
    cz coords(du: a, dv: b) std.top
    along_z := circle(center: cz) hint(r: 2.5) in std.top
    radius(r) along_z
    drill_z := solid(std.solids.Through(face(along_z)))
    drill_z cut part
  }
}
