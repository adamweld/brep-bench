// The solid a ring lying flat fills as it slides across by less than the width of its hole. The
// ring is a torus about the z axis, its tube (radius 2) 6 from the axis on z = 0: the tube's circle
// drawn in the front plane and turned about z. The part is the kernel's sweep of the ring as it
// slides 6 along +x without turning.
unit mm
use std
use std.solids
use std.place (vertical, right_of)
use std.motion (slides)

in std.front {
  construction centerline spindle := vertical line(std.origin, hint((0, 1)))
  length(1mm) spindle
  private tc := point hint((6, 0))
  tc right_of(d: 6mm) std.origin
  private tube := circle(center: tc) hint(r: 2)
  radius(2mm) tube
}
construction ring := solid(std.solids.Revolution(face(tube), about: spindle))

// the slide: 6 along x over the roll's one turn
carriage := frame
carriage slides(6mm) std.x
part := solid({ p | some t in (0deg, 360deg): p in ring at(under: carriage) t })
