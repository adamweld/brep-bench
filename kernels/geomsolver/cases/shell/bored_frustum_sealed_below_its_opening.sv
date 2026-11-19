// A cone frustum bored along its axis, as a turn of its half-section: in the front plane (x, z),
// the quadrilateral from the bore's radius 1 out to radius 6 at z = 0 and radius 3 at z = 4, turned
// a whole turn about the z axis. Shelled to a wall 1 thick and open through its top, `part.top`:
// the side, the base and the bore moved in 1 close on one another 1/3 below the top, so the cavity
// is a ring sealed below the opening.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.front {
  o := point
  fix((0, 0)) o
  a := point hint((1, 0))
  b := point hint((6, 0))
  c := point hint((3, 4))
  d := point hint((1, 4))
  horizontal (base := line(a, b)) -> (side := line(b, c)) -> horizontal (top := line(c, d)) ->
    vertical (bore := line(d, a)) -> close
  o level(up) a
  o distance(1) a
  o distance(6) b
  a distance(4) d
  d distance(2) c
}

part := solid(std.solids.Revolution(face(base, side, top, bore), about: std.z))
part.top shell(t: 1mm) part
