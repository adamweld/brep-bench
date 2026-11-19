// The part of the ball of radius 5 about the origin below the plane z = 4.5, as a turn of its
// section: in the front plane (x, z), the axis from the pole up to z = 4.5, the flat out to the
// sphere and the arc of radius 5 back down to the pole, turned a whole turn about the z axis.
// Shelled to a wall 1 thick and open through its flat face, `part.flat`: the sphere moved in 1,
// the sphere of radius 4, lies wholly below the opening, so the cavity is sealed.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.front {
  o := point
  fix((0, 0)) o
  s := point hint((0, -5))
  m := point hint((0, 4.5))
  r := point hint((2.18, 4.5))
  vertical (axle := line(s, m))
  horizontal (flat := line(m, r))
  (bowl := arc(center: o, from: s, to: r))
  radius(5mm) bowl
  o level(right) m
  o distance(4.5mm, along: up) m
}

part := solid(std.solids.Revolution(face(axle, flat, bowl), about: std.z))
part.flat shell(t: 1mm) part
