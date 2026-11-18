// A solid cone frustum as a turn of its half-section: in the front plane (x, z), the quadrilateral
// from the axis out to radius 6 at z = 0 and radius 3 at z = 4, turned a whole turn about the z
// axis. Shelled to a wall 2.7 thick and open through its top, `part.top`: the side moved in closes
// to its apex 0.5 below the top, so the cavity, a cone standing on the base moved up, never reaches
// the opening and is sealed.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.front {
  o := point
  fix((0, 0)) o
  b := point hint((6, 0))
  c := point hint((3, 4))
  d := point hint((0, 4))
  horizontal (base := line(o, b)) -> (side := line(b, c)) -> horizontal (top := line(c, d)) ->
    vertical (axle := line(d, o)) -> close
  o distance(6) b
  o distance(4) d
  d distance(3) c
}

part := solid(std.solids.Revolution(face(base, side, top, axle), about: std.z))
part.top shell(t: 2.7mm) part
