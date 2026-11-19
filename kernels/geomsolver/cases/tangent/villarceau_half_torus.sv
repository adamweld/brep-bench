// A torus about the z axis, its tube's circle of radius 3 drawn in the side plane (y, z) 5 out
// along y and turned a whole turn about z, so its seam lies in the plane x = 0 on the side y > 0,
// cut by the bitangent plane 3x - 4z = 0. The cutter is a block over that plane: its section drawn
// in the front plane (x, z), one side along the plane's trace through the origin, rising 3 in z for
// every 4 in x, and the square stood on it toward +z; the prism over it from y = -10 to y = 10
// reaches past the torus. What lies on and below the plane is kept.
unit mm
use std
use std.solids
use std.solids (cut)

in std.side {
  c := point
  fix((5, 0)) c
  tube := circle(center: c) hint(r: 3)
  radius(3) tube
}

in std.front {
  o := point
  fix((0, 0)) o
  a := point hint((-12, -9))
  b := point hint((12, 9))
  c2 := point hint((3, 21))
  d := point hint((-21, 3))
  block := (sole := line(a, b)) -> (rise := line(b, c2)) -> (over := line(c2, d)) ->
           (fall := line(d, a)) -> close
  o distance(12, along: u) b
  o distance(9, along: v) b
  o midpoint sole
  sole perpendicular(sense: ccw) rise
  rise perpendicular(sense: ccw) over
  rise equal sole
  over equal sole
}

above := solid(std.solids.Prism(block, from: -10mm, to: 10mm))
part := solid(std.solids.Revolution(face(tube), about: std.z))
above cut part
