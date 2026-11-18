// A ring, everything between radii 3 and 8 about the z axis with z within 2.5: its section in the
// front plane (x, z) turned whole about the z axis. A drum, the cylinder of radius 1 and length 3
// whose axis leans 25 degrees from upright in the plane y = 1.2, along (sin 25, 0, cos 25), its
// middle (5, 1.2, 0): its end disc drawn in the plane through its middle whose u is x turned 25
// degrees about y and v is y, carried 1.5 either way along that plane's normal. It is turned
// rigidly 100 degrees about the z axis, right-handed (from +x toward +y), wholly inside the ring,
// and everything it passes through cut from the ring: a sealed channel.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (turns)

in std.front {
  a := point
  b := point hint((8, -2.5))
  c := point hint((8, 2.5))
  d := point hint((3, 2.5))
  meridian := horizontal line(a, b) -> vertical line(b, c) ->
              horizontal line(c, d) -> vertical line(d, a) -> close
  fix((3, -2.5)) a
  a distance(5) b
  b distance(5) c
}
part := solid(std.solids.Revolution(meridian, about: std.z))

across := axis hint(dir: (0.9, 0, -0.42))
across perpendicular std.y
std.x angle(25deg, about: std.y) across
along := axis hint(dir: (0, 1, 0))
along parallel(sense: same) std.y
waist := plane(u: across, v: along)
fix(origin == (5, 1.2, 0)) waist
in waist {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 1)
  radius(1) rim
}
drum := solid(std.solids.Prism(face(rim), from: -1.5mm, to: 1.5mm))

swing := frame
swing turns(1) std.z
channel := solid({ p | some t in (0deg, 100deg): p in drum at(under: swing) t })
channel cut part
