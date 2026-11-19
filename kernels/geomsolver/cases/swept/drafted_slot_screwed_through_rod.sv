// A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20; a drafted bar pointing at
// the rod's axis: its section the slot in the plane x = 12 made of the half discs of radius 1 about
// (12, +-1.5, 0) and the rectangle |y| <= 1.5, |z| <= 1 between them, carried along -x to x = 4 and
// drafted 3 degrees inward from that plane; carried by a screw about the z axis that turns
// right-handed (from +x toward +y) and rises 6 a turn, from a turn of -90 degrees to one of 1350,
// and everything it passes through cut from the rod.
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

// the plane of the slot, x = 12, its u along y and its v along z, so its normal is +x
across := axis hint(dir: (0, 1, 0))
across parallel(sense: same) std.y
up := axis hint(dir: (0, 0, 1))
up parallel(sense: same) std.z
butt := plane(u: across, v: up)
fix(origin == (12, 0, 0)) butt
in butt {
  left := point
  right := point
  fix((-1.5, 0)) left
  fix((1.5, 0)) right
  slot := horizontal line(hint((-1.5, -1)), hint((1.5, -1))) -> tangent
          (east := arc(center: right) hint(r: 1)) -> tangent
          horizontal line(hint((1.5, 1)), hint((-1.5, 1))) -> tangent
          (west := arc(center: left) hint(r: 1)) -> tangent close
  radius(1) east
  radius(1) west
}
bar := solid(std.solids.Drafted(slot, from: -8mm, to: 0mm, draft: 3deg))

climb := frame
climb screws(6mm) std.z
groove := solid({ p | some t in (-90deg, 1350deg): p in bar at(under: climb) t })
groove cut part
