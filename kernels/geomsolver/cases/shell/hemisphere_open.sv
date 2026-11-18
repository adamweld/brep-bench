// The lower half of the ball of radius 5 about the origin, as a turn of its quarter disc: in the
// front plane (x, z), the flat from the centre out to radius 5, the quarter circle down to the
// pole and the axis back up, turned a whole turn about the z axis. Shelled to a wall 1 thick and
// open through its flat face, `part.flat`: a bowl.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.front {
  o := point
  fix((0, 0)) o
  e := point hint((5, 0))
  s := point hint((0, -5))
  horizontal (flat := line(o, e))
  (bowl := arc(center: o, from: s, to: e))
  vertical (axle := line(s, o))
  radius(5mm) bowl
}

part := solid(std.solids.Revolution(face(flat, bowl, axle), about: std.z))
part.flat shell(t: 1mm) part
