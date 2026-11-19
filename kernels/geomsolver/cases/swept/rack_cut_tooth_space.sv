// One tooth space of a spur gear (module 2, 20 teeth, 20 degree pressure angle) cut by a rack
// tooth. The blank is the disc of radius 22 about the y axis, drawn in the front plane (x, z) and
// extruded from y = 0 to 6. The tooth is the trapezoid with corners (18, +-0.842856) and
// (24, +-3.026666) in (x, z), extruded from y = -2 to 8. The blank turns about y while the rack
// slides along its pitch line x = 20, 2 pi 20 a turn, so the line rolls on the pitch circle; the
// kernel's sweep of the tooth as the blank sees it, over -60..60 degrees, is cut from the blank.
unit mm
use std
use std.solids (cut)
use std.place (vertical)
use std.motion (spins, slides)

in std.front {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 22)
  radius(22mm) rim

  // the pitch line, tangent to the pitch circle (radius 20) at the middle of the tooth
  s0 := point
  fix((20, 0)) s0
  construction pitch_line := vertical line(s0, hint((20, 10)))
  length(10mm) pitch_line

  // the rack's tooth, as given
  t0 := point hint((18, -0.84))
  t1 := point hint((18, 0.84))
  t2 := point hint((24, 3.03))
  t3 := point hint((24, -3.03))
  fix((18, -0.842856)) t0
  fix((18, 0.842856)) t1
  fix((24, 3.026666)) t2
  fix((24, -3.026666)) t3
}
// the front plane's normal is -y, so the ordinates along it are minus y
part := solid(std.solids.Prism(face(rim), from: -6mm, to: 0mm))
construction tooth := solid(std.solids.Prism(face(t0, t1, t2, t3, -> close), from: -8mm, to: 2mm))

// the blank turning about its centre, and the rack sliding a pitch circumference a turn
turn := frame
turn spins(1) o
rack := frame
rack slides(2 * pi * 20mm) pitch_line
construction space := solid({ p | some t in (-60deg, 60deg): p in tooth at(under: rack, seen_from: turn) t })
space cut part
