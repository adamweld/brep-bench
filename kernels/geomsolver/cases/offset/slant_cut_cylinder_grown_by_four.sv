// A cylinder of radius 3 about the z axis, from its flat bottom at z = −10 up to the plane through
// the origin leaning at 45° to the bottom, z = x·tan 45°, grown by 4 with rounded edges. The cut
// cylinder is the region within 3 of the axis, above the plane z = −10 and behind the slanted plane
// (behind its normal, the axis `lean`, which is z turned 45° about y toward −x); grown, it is the
// points within 4 of it, `std.rounds.Grown`'s body.
unit mm
use std
use std.sets
use std.rounds

o := point
fix((0, 0, 0)) o
up := point
fix((0, 0, 1)) up
z_axis := line(o, up)
lean := axis hint(dir: (-0.5, 0, 0.87))
lean perpendicular std.y
lean angle(45deg, about: std.y) std.z

cut_cylinder := solid({ p |
  p in std.sets.Cylinder(z_axis, r: 3mm)
  p distance(>= -10mm, along: n) std.top
  p in std.sets.HalfSpace(o, lean, d: 0mm)
})
grown := std.rounds.Grown(cut_cylinder, d: 4mm)
part := solid(grown.body)
