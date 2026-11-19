// The solid a cone fills as it is spun partway about a line crossing its axis aslant: the cone
// about the line through the origin along w = (cos 30, 0, sin 30), its point at -2w, opening at
// 50 degrees (each side 25 degrees from its axis) to its flat base at +5w; a turn of its
// half-section in the front plane (x, z) about its axis. It is turned rigidly 150 degrees about
// the z axis, right-handed (from +x toward +y).
unit mm
use std
use std.solids
use std.motion (turns)

in std.front {
  o := point
  fix((0, 0)) o
  apex := point hint((-1.73, -1))
  centre := point hint((4.33, 2.5))
  rim := point hint((2.69, 5.34))
  spine := line(apex, centre)
  o coincident spine
  o distance(2) apex
  o distance(5) centre
  // the axis 30 degrees up from x, turned about the front plane's normal (-y)
  std.x angle(30deg, about: std.front.normal) spine
  half := (flank := line(apex, rim)) -> (base := line(rim, centre)) ->
          (axle := line(centre, apex)) -> close
  base perpendicular spine
  spine angle(25deg) flank
}
cone := solid(std.solids.Revolution(half, about: axle))

swing := frame
swing turns(1) std.z
part := solid({ p | some t in (0deg, 150deg): p in cone at(under: swing) t })
