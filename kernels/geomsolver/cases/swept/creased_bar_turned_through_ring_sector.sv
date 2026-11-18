// A curved block, the ring sector between radii 8 and 12 about the z axis, z within 1, polar angle
// from 0 to 60 degrees: its section in the front plane (x, z) turned 60 degrees about the z axis.
// The bar is drawn in the plane through the origin whose u is x and whose v = (0, cos 20, -sin 20)
// (y turned -20 degrees about x), leaning 20 degrees across the turn: its section the rectangle
// 10 <= u <= 13, |v| <= 2 there, carried 4 either way along the plane's normal and drafted 2
// degrees inward both ways from the plane, a convex crease along it. It is turned rigidly about the
// z axis from -140 to +140 degrees, and everything it passes through cut from the block.
unit mm
use std
use std.solids
use std.solids (cut)
use std.motion (turns)

in std.front {
  a := point
  b := point hint((12, -1))
  c := point hint((12, 1))
  d := point hint((8, 1))
  meridian := horizontal line(a, b) -> vertical line(b, c) ->
              horizontal line(c, d) -> vertical line(d, a) -> close
  fix((8, -1)) a
  a distance(4) b
  b distance(2) c
}
part := solid(std.solids.Revolution(meridian, about: std.z, to: 60deg))

along := axis hint(dir: (0, 0.94, -0.34))
along perpendicular std.x
std.y angle(-20deg, about: std.x) along
across := axis hint(dir: (1, 0, 0))
across parallel(sense: same) std.x
holder := plane(u: across, v: along)
fix(origin == (0, 0, 0)) holder
in holder {
  b0 := point
  b1 := point hint((13, -2))
  b2 := point hint((13, 2))
  b3 := point hint((10, 2))
  section := horizontal line(b0, b1) -> vertical line(b1, b2) ->
             horizontal line(b2, b3) -> vertical line(b3, b0) -> close
  fix((10, -2)) b0
  b0 distance(3) b1
  b1 distance(4) b2
}
bar := solid(std.solids.Drafted(section, from: -4mm, to: 4mm, draft: 2deg))

swing := frame
swing turns(1) std.z
groove := solid({ p | some t in (-140deg, 140deg): p in bar at(under: swing) t })
groove cut part
