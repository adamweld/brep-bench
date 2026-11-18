// Half a solid torus: its tube's circle of radius 2, drawn in the front plane (x, z) 6 out along x,
// turned a half turn about the z axis from the x axis through y > 0, so its ends are capped
// `part.start` and `part.end`. Shelled to a wall 0.5 thick and open through both ends: a curved
// pipe.
//
// `std.rounds`' `A shell(t: …) B` takes one face path as its opening, so with two openings its
// own words are written out here: the cavity is the half torus's inside less the wall within 0.5
// of the one face kept, the tube, and it is cut from the half torus.
unit mm
use std
use std.solids
use std.solids (cut)

in std.front {
  c := point
  fix((6, 0)) c
  tube := circle(center: c) hint(r: 2)
  radius(2) tube
}

part := solid(std.solids.Revolution(face(tube), about: std.z, to: 180deg))
within := { q | q in part }
cavity := solid(within)
repeat w in part {
  when w != part.start and w != part.end {
    wall := { p | p distance(<= 0.5mm) w }
    walled := solid(wall)
    cavity and { p | p not in walled }
  }
}
cavity cut part
