// A pin lug: the plate's 80 × 40 outline drawn in the front plane (x, z) and the prism over it 10
// thick, y from −5 to 5; the pin hole of radius 8 about (60, 20) cut by a prism of its circle that
// runs past both faces.
unit mm
use std
use std.solids (cut)

l := 80mm
w := 40mm
t := 10mm
d := 16mm

in std.front {
  a := point
  b := point hint((80, 0))
  c := point hint((80, 40))
  e := point hint((0, 40))
  fix((0mm, 0mm)) a
  outline := horizontal (low := line(a, b)) -> vertical (end := line(b, c)) ->
             horizontal (high := line(c, e)) -> vertical (root := line(e, a)) -> close
  length(l) low
  length(w) end
  centre := point hint((60, 20))
  b distance(w / 2, along: left) centre     // the hole's axis w/2 in from the end ...
  b distance(w / 2, along: up) centre       // ... and half way up it
  bore := radius(d / 2) circle(center: centre) hint(r: 8)
}
part := solid(std.solids.Prism(outline, from: -t / 2, to: t / 2))
pin := solid(std.solids.Prism(face(bore), from: -t, to: t))
pin cut part
