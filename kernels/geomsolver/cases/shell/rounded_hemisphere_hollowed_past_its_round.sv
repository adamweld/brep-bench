// The upper half of the ball of radius 5 about the origin, as a turn of its quarter disc: in the
// front plane (x, z), the flat from the centre out to radius 5, the quarter circle up to the pole
// and the axis back down, turned a whole turn about the z axis. Its rim, where the flat meets the
// dome, is rounded to radius 1, and it is hollowed to walls 2 thick, more than the round's radius,
// with no opening: the round moved in turns inside out and drops, and the closed cavity is the
// hemisphere shrunk by 2 with a sharp edge. The shell reads the hemisphere as its round leaves it,
// a solid of the rounded hemisphere.
unit mm
use std
use std.solids
use std.rounds (round, shell)

in std.front {
  o := point
  fix((0, 0)) o
  e := point hint((5, 0))
  n := point hint((0, 5))
  horizontal (flat := line(o, e))
  (dome := arc(center: o, from: e, to: n))
  vertical (axle := line(n, o))
  radius(5mm) dome
}

half := solid(std.solids.Revolution(face(flat, dome, axle), about: std.z))
half.flat round(r: 1mm) half.dome

part := solid(half)                 // the hemisphere as its round leaves it
shell(t: 2mm) part
