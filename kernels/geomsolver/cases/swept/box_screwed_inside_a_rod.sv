// A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20; a small box,
// 2.5 <= x <= 3.5, |y| <= 0.5, 2 <= z <= 3, carried by a screw about the z axis that turns
// right-handed (from +x toward +y) and rises 4 a turn, for two whole turns wholly inside the rod,
// and everything it passes through cut from the rod: a sealed helical tunnel.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (screws)

in std.top {
  centreline := point
  fix((0, 0)) centreline
  skin := circle(center: centreline) hint(r: 5)
  radius(5) skin
}
part := solid(std.solids.Prism(face(skin), from: 0mm, to: 20mm))

in std.top {
  e := point
  f := point hint((3.5, -0.5))
  g := point hint((3.5, 0.5))
  h := point hint((2.5, 0.5))
  footprint := horizontal line(e, f) -> vertical line(f, g) ->
               horizontal line(g, h) -> vertical line(h, e) -> close
  fix((2.5, -0.5)) e
  e distance(1) f
  f distance(1) g
}
box := solid(std.solids.Prism(footprint, from: 2mm, to: 3mm))

climb := frame
climb screws(4mm) std.z
tunnel := solid({ p | some t in (0deg, 720deg): p in box at(under: climb) t })
tunnel cut part
