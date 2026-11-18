// A ring lying flat, turned a whole turn about the z axis beside it. The ring is a torus about the
// vertical line through (8, 0), its tube (radius 1) 3 from that line on z = 0: the tube's circle
// drawn in the front plane and turned about the line. The part is the kernel's sweep of the ring
// as it turns a whole turn about z.
unit mm
use std
use std.solids
use std.place (vertical, right_of)
use std.motion (turns)

in std.front {
  construction centerline spindle := vertical line(std.origin, hint((0, 1)))
  length(1mm) spindle
  private foot := point
  foot right_of(d: 8mm) std.origin
  construction centerline ring_axis := vertical line(foot, hint((8, 1)))
  length(1mm) ring_axis
  private tc := point hint((11, 0))
  tc right_of(d: 3mm) foot
  private tube := circle(center: tc) hint(r: 1)
  radius(1mm) tube
}
construction ring := solid(std.solids.Revolution(face(tube), about: ring_axis))

swing := frame
swing turns(1) spindle
part := solid({ p | some t in (0deg, 360deg): p in ring at(under: swing) t })
