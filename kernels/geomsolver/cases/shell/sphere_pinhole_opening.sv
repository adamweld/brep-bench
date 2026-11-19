// The part of the ball of radius 5 about the origin below the plane z = 3.9999, as a turn of its
// section: in the front plane (x, z), the axis from the pole up to z = 3.9999, the flat out to the
// sphere and the arc of radius 5 back down to the pole, turned a whole turn about the z axis.
// Shelled to a wall 1 thick and open through its flat face, `part.flat`: the sphere moved in 1
// reaches 0.0001 above the opening's plane, so the cavity opens through a pinhole.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.front {
  o := point
  fix((0, 0)) o
  s := point hint((0, -5))
  m := point hint((0, 3.9999))
  r := point hint((3, 3.9999))
  vertical (axle := line(s, m))
  horizontal (flat := line(m, r))
  (bowl := arc(center: o, from: s, to: r))
  radius(5mm) bowl
  o level(right) m
  o distance(3.9999mm, along: up) m
}

part := solid(std.solids.Revolution(face(axle, flat, bowl), about: std.z))
part.flat shell(t: 1mm) part
