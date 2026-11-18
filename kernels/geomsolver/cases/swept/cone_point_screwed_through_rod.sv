// A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20; a pointed cone about the x
// axis, its point (4, 0, 0), opening away from the rod at 60 degrees (each side 30 degrees from its
// axis) to a flat base at x = 12, a turn of its half-section in the front plane; carried by a screw
// about the z axis that turns right-handed (from +x toward +y) and rises 6 a turn, from a turn of
// -90 degrees to one of 1350, and everything it passes through cut from the rod.
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

in std.front {
  tip := point
  rim := point hint((12, 4.6))
  centre := point hint((12, 0))
  fix((4, 0)) tip
  tip distance(8) centre
  half := (flank := line(tip, rim)) -> vertical line(rim, centre) ->
          horizontal (axle := line(centre, tip)) -> close
  flank angle(30deg) std.x
}
cone := solid(std.solids.Revolution(half, about: axle))

climb := frame
climb screws(6mm) std.z
groove := solid({ p | some t in (-90deg, 1350deg): p in cone at(under: climb) t })
groove cut part
