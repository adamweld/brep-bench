// A cone frustum as a turn of its half-section: in the front plane (x, z), the quadrilateral from
// the axis out to radius 6 at z = 0 and radius 3 at z = 6, turned a whole turn about the z axis,
// its bottom rim, where the base meets the side, rounded to radius 1.5. Shelled to walls 2 thick,
// more than the round's radius, and open through its top: the round moved in turns inside out and
// drops, and the cavity has the sharp frustum's corners. The shell reads the frustum as its round
// leaves it, a solid of the rounded frustum.
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
frustum.base round(r: 1.5mm) frustum.side

part := solid(frustum)              // the frustum as its round leaves it
part.frustum.top shell(t: 2mm) part
