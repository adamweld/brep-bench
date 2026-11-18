// A paraboloid: in the front plane (x, z), the parabola x² = 8z traced from its vertex at the
// origin out to x = 8, the line in from there to the z axis at z = 8, and the axis back down,
// turned a whole turn about the z axis.
unit mm
use std
use std.solids

// the point of the parabola `u` out from its vertex `o`
component Parabola(o: point, u: Length) {
  p := point(x: o.x + u, y: o.y + u * u / 8mm)
}

in std.front {
  o := point
  fix((0, 0)) o
  rim := point hint((8, 8))
  top := point hint((0, 8))
  o coincident k hint(t: 0)
  rim coincident k hint(t: 8)
  fix(x == 8mm) rim
  horizontal (lid := line(rim, top))
  construction centerline spindle := line(top, o)
  vertical spindle
}
k := trace(Parabola(o).p over u in (0mm, 8mm))

part := solid(std.solids.Revolution(face(k from o to rim, lid, spindle), about: std.z))
