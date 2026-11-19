// A cone with its tip rounded, as a turn of its half-section: in the front plane (x, z), the base
// out to radius 6, the side up toward the apex on the axis 12 above the base, and the tip, an arc
// of radius 1 about a centre on the axis going on tangent from the side to the axis, turned a whole
// turn about the z axis. Hollowed to a wall 1.5 thick with no opening: the tip's ball moved in is
// less than nothing, so the cavity is a plain cone, the side moved in run on to its own apex.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.front {
  o := point
  fix((0, 0)) o
  b := point hint((6, 0))
  apex := point hint((0, 12))
  t := point hint((0.894, 10.21))
  k := point hint((0, 9.76))
  top := point hint((0, 10.76))
  construction cone := line(b, apex)
  horizontal (base := line(o, b))
  (side := line(b, t))
  (tip := arc(center: k, from: t, to: top))
  vertical (axle := line(top, o))
  // the cone: radius 6 at its base, its apex on the axis 12 up
  o distance(6) b
  o level(right) apex
  o distance(12mm, along: up) apex
  t coincident cone
  // the tip: a ball of radius 1 on the axis, tangent to the side
  k coincident axle
  radius(1mm) tip
  side tangent tip
}

part := solid(std.solids.Revolution(face(base, side, tip, axle), about: std.z))
shell(t: 1.5mm) part
