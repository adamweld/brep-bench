// A cone frustum as a turn of its half-section: in the front plane (x, z), the quadrilateral from
// the axis out to radius 6 at z = 0 and radius 3 at z = 6, turned a whole turn about the z axis,
// both its rims, where the side meets the base and the top, rounded to radius 1.5. Hollowed to
// walls 2 thick, more than the rounds' radius, with no opening: the rounds moved in turn inside
// out and drop, and the closed cavity is the sharp frustum shrunk by the wall. The shell reads the
// frustum as its rounds leave it, a solid of the rounded frustum.
unit mm
use std
use std.solids
use std.rounds (round, shell)

in std.front {
  o := point
  fix((0, 0)) o
  b := point hint((6, 0))
  c := point hint((3, 6))
  d := point hint((0, 6))
  horizontal (base := line(o, b)) -> (side := line(b, c)) -> horizontal (top := line(c, d)) ->
    vertical (axle := line(d, o)) -> close
  o distance(6) b
  o distance(6) d
  d distance(3) c
}

frustum := solid(std.solids.Revolution(face(base, side, top, axle), about: std.z))
frustum.side round(r: 1.5mm) frustum.base
frustum.side round(r: 1.5mm) frustum.top

part := solid(frustum)              // the frustum as its rounds leave it
shell(t: 2mm) part
