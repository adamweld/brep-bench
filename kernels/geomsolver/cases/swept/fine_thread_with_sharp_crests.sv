// A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20; a bar, the box
// 4.5 <= x <= 12, |y| <= 8, |z| <= 0.45, carried by a screw about the z axis that turns
// right-handed (from +x toward +y) and rises 1 a turn, from a turn of -180 degrees to one of 7380,
// and everything it passes through cut from the rod: a thread so fine its crests are sharp.
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
  f := point hint((12, -8))
  g := point hint((12, 8))
  h := point hint((4.5, 8))
  footprint := horizontal line(e, f) -> vertical line(f, g) ->
               horizontal line(g, h) -> vertical line(h, e) -> close
  fix((4.5, -8)) e
  e distance(7.5) f
  f distance(16) g
}
bar := solid(std.solids.Prism(footprint, from: -0.45mm, to: 0.45mm))

climb := frame
climb screws(1mm) std.z
thread := solid({ p | some t in (-180deg, 7380deg): p in bar at(under: climb) t })
thread cut part
