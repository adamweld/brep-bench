// A hyperboloid of one sheet: in the front plane (x, z), the branch x > 0 of the hyperbola
// x²/16 - z²/36 = 1 traced from z = -6 to z = 6, its waist 4 out from the axis; with the lines in
// to the z axis at each end and the axis between them, turned a whole turn about the z axis.
unit mm
use std
use std.solids

// the point of the hyperbola `u` up from its waist on the line through `o`
component Hyperbola(o: point, u: Length) {
  p := point(x: o.x + 4mm * sqrt(1 + (u / 6mm)^2), y: o.y + u)
}

in std.front {
  o := point
  fix((0, 0)) o
  low := point hint((5.66, -6))
  high := point hint((5.66, 6))
  low coincident k hint(t: -6)
  high coincident k hint(t: 6)
  fix(y == -6mm) low
  fix(y == 6mm) high
  foot := point hint((0, -6))
  head := point hint((0, 6))
  horizontal (top := line(high, head))
  construction centerline spindle := line(head, foot)
  vertical spindle
  horizontal (bottom := line(foot, low))
  fix(x == 0mm) foot
}
k := trace(Hyperbola(o).p over u in (-6mm, 6mm))

part := solid(std.solids.Revolution(face(k from low to high, top, spindle, bottom), about: std.z))
