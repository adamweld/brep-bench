// A cube from the origin to (20, 20, 20), and a notch, the cube from (10, 10, 10) to (20, 20, 20),
// cut from its corner; every edge among the faces the notch gave the part — its three concave
// inside edges — rounded to radius 3, filled, a ball's corner where the three meet.
unit mm
use std
use std.solids
use std.solids (cut)
use std.rounds (round)

in std.top {
  block_sq := step(u: 20mm, v: 0mm) (bs := line) -> step(v: 20mm) (be := line) ->
              step(u: 0mm) (bn := line) -> step(v: 0mm) (bw := line) -> close
  notch_sq := step(u: 20mm, v: 10mm) (ns := line) -> step(v: 20mm) (ne := line) ->
              step(u: 10mm) (nn := line) -> step(v: 10mm) (nw := line) -> close
}

notch := solid(std.solids.Prism(notch_sq, from: 10mm, to: 20mm))
part := solid(std.solids.Prism(block_sq, from: 0mm, to: 20mm))
notch cut part
round(r: 3mm) part.notch
