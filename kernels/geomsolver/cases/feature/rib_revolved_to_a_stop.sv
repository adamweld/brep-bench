// A hub, a stop and a rib, one solid. The hub: a circle of radius 10 in the top plane and the
// prism over it to z = 10. The stop: the rectangle out to 25 and 10 tall in the front plane (x, z),
// turned about the z axis between the half-planes at 120° and 135°. The rib: the rectangle
// 5 ≤ x ≤ 20, 0 ≤ z ≤ 4 in the front plane, turned about z from 0° toward +y until it meets the
// stop's face — Solvent's turn has no "up to a face", so it is turned to the angle that face
// stands at, read from the stop's own number.
unit mm
use std
use std.solids
use std.solids (union)

stop_from := 120deg
stop_to := 135deg

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 10)
  radius(10mm) rim
}
part := solid(std.solids.Prism(face(rim), from: 0mm, to: 10mm))

in std.front {
  a := point
  fix((0mm, 0mm)) a
  wedge := step(u: 25mm) (w0 := line(a)) -> step(v: 10mm) (w1 := line) ->
           step(u: 0mm) (w2 := line) -> (w3 := line(p2: a)) -> close
  b := point
  a distance(5mm, along: u) b
  a level(up) b
  rib_section := step(u: 20mm) (r0 := line(b)) -> step(v: 4mm) (r1 := line) ->
                 step(u: 5mm) (r2 := line) -> (r3 := line(p2: b)) -> close
}
stop := solid(std.solids.Revolution(wedge, about: std.z, from: stop_from, to: stop_to))
stop union part
rib := solid(std.solids.Revolution(rib_section, about: std.z, to: stop_from))
rib union part
