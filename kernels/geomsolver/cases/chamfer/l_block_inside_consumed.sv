// An L-shaped block: its section drawn in the front plane (x, z) through (0, 0), (10, 0), (10, 3),
// (3, 3), (3, 10) and (0, 10), and the prism over it from y = 0 to y = 10 (the front's normal is
// -y). Its one concave edge, where the face swept from `ledge` (z = 3) meets the face swept from
// `riser` (x = 3), is chamfered 7 on each face, adding material: the flat runs from the far edge of
// one face to the far edge of the other, and both faces are consumed.
unit mm
use std
use std.solids
use std.rounds (chamfer)

in std.front {
  o := point
  fix((0, 0)) o
  a := point hint((10, 0))
  b := point hint((10, 3))
  c := point hint((3, 3))
  d := point hint((3, 10))
  e := point hint((0, 10))
  horizontal (floor := line(o, a)) -> vertical (toe := line(a, b)) ->
  horizontal (ledge := line(b, c)) -> vertical (riser := line(c, d)) ->
  horizontal (top := line(d, e)) -> vertical (back := line(e, o)) -> close
  o distance(10) a
  a distance(3) b
  o distance(3, along: u) c
  o distance(10) e
  section := face(floor, toe, ledge, riser, top, back)
}

part := solid(std.solids.Prism(section, from: -10mm, to: 0mm))
part.ledge chamfer(d: 7mm) part.riser
