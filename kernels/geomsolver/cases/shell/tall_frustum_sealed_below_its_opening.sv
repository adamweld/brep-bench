// A tall solid cone frustum as a turn of its half-section: in the front plane (x, z), the
// quadrilateral from the axis out to radius 7 at z = 0 and radius 1 at z = 8, turned a whole turn
// about the z axis. Shelled to a wall 2 thick and open through its narrow top, `part.top`: the side
// moved in closes to its apex 2 below the top, so the cavity is a cone sealed well below the
// opening.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.front {
  o := point
  fix((0, 0)) o
  b := point hint((7, 0))
  c := point hint((1, 8))
  d := point hint((0, 8))
  horizontal (base := line(o, b)) -> (side := line(b, c)) -> horizontal (top := line(c, d)) ->
    vertical (axle := line(d, o)) -> close
  o distance(7) b
  o distance(8) d
  d distance(1) c
}

part := solid(std.solids.Revolution(face(base, side, top, axle), about: std.z))
part.top shell(t: 2mm) part
