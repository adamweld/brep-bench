// A cap of a hyperboloid of two sheets: in the front plane (x, z), the branch z > 0 of the
// hyperbola z²/9 - x²/16 = 1 traced from its vertex (0, 3) out to where it reaches z = 6, the line
// in from there to the z axis, and the axis back down to the vertex, turned a whole turn about the
// z axis.
unit mm
use std
use std.solids

// the point of the hyperbola `u` out from the axis through `o`
component Hyperbola(o: point, u: Length) {
  p := point(x: o.x + u, y: o.y + 3mm * sqrt(1 + (u / 4mm)^2))
}

in std.front {
  o := point
  fix((0, 0)) o
  vertex := point hint((0, 3))
  rim := point hint((6.93, 6))
  vertex coincident k hint(t: 0)
  rim coincident k hint(t: 6.93)
  fix(x == 0mm) vertex
  fix(y == 6mm) rim
  top := point hint((0, 6))
  horizontal (lid := line(rim, top))
  construction centerline spindle := line(top, vertex)
  vertical spindle
}
k := trace(Hyperbola(o).p over u in (0mm, 8mm))

part := solid(std.solids.Revolution(face(k from vertex to rim, lid, spindle), about: std.z))
