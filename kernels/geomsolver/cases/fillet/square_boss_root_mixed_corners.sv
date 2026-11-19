// A plate, the box from the origin to (40, 30, 5), and a square boss, the box from (12, 7, 5) to
// (28, 23, 15), their footprints drawn in the top plane. The rounds are made one after another,
// as the case says: first the boss's four upright edges — where two of its faces that both run
// along z meet — rounded to radius 3; then the boss as those rounds leave it joined to the plate,
// and its root — every edge where the boss's faces, its rounds' among them, meet the plate's —
// rounded to radius 2, filled: one tangent run round the rounded square.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds (round)

in std.top {
  plate_sq := step(u: 40mm, v: 0mm) (ps := line) -> step(v: 30mm) (pe := line) ->
              step(u: 0mm) (pn := line) -> step(v: 0mm) (pw := line) -> close
  boss_sq := step(u: 28mm, v: 7mm) (bs := line) -> step(v: 23mm) (be := line) ->
             step(u: 12mm) (bn := line) -> step(v: 7mm) (bw := line) -> close
}

plate := solid(std.solids.Prism(plate_sq, from: 0mm, to: 5mm))
boss := solid(std.solids.Prism(boss_sq, from: 5mm, to: 15mm))

// 1. the upright edges
repeat f in boss {
  repeat g in boss {
    when f parallel std.z and g parallel std.z and f != g {
      f round(r: 3mm) g
    }
  }
}

// 2. the root
part := solid(plate)
boss union part
part.boss round(r: 2mm) part.plate
