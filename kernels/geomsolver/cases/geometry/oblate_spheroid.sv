// An oblate spheroid: the half of the ellipse of semi-axes 6 along x and 3 along z about the
// origin of the front plane (x, z) that lies on the side x >= 0, closed by its chord on the z
// axis, turned a whole turn about the z axis.
unit mm
use std
use std.shapes
use std.solids

in std.front {
  o := point
  fix((0, 0)) o
  south := point hint((0, -3))
  north := point hint((0, 3))
  south coincident rim hint(t: -90)
  north coincident rim hint(t: 90)
  fix(x == 0mm) south
  fix(x == 0mm) north
  construction centerline spindle := line(south, north)
}
rim := trace(std.shapes.Ellipse(o, a: 6mm, b: 3mm, tilt: 0deg).p over u in (-90deg, 90deg))

part := solid(std.solids.Revolution(face(rim from south to north, spindle), about: std.z))
