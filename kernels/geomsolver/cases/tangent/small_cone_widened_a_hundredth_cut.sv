// A small cone, its base of radius 0.00075 at z = 0 and its apex 0.0015 above on the z axis, and a
// copy whose base is a hundredth wider, of radius 0.0007575, with the same base plane and apex: each
// the half-section, a right triangle drawn in the front plane (x, z) from the axis, turned a whole
// turn about the z axis. The copy less the cone.
unit mm
use std
use std.solids
use std.solids (cut)

in std.front {
  o := point
  apex := point hint((0, 0.0015))
  rim := point hint((0.00075, 0))
  wide_rim := point hint((0.0007575, 0))
  fix((0, 0)) o
  o distance(0.0015) apex
  o distance(0.00075) rim
  o distance(0.0007575) wide_rim

  narrow := horizontal line(o, rim) -> line(rim, apex) -> vertical line(apex, o) -> close
  wide := horizontal line(o, wide_rim) -> line(wide_rim, apex) -> line(apex, o) -> close
}

cone := solid(std.solids.Revolution(narrow, about: std.z))
part := solid(std.solids.Revolution(wide, about: std.z))
cone cut part
