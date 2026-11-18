// An L-block: its section drawn in the front plane (x, z), the polygon through (0, 0), (40, 0),
// (40, 10), (15, 10), (15, 30) and (0, 30), and the prism behind it to y = 20. Its riser is the
// face swept from `riser`, at x = 15. Then the riser moved out by 10, along +x.
//
// Solvent moves no face of a built solid: a solid is the points of a set, and nothing about it is
// solved for. The riser is its edge of the section swept along y, and the edge stands where the
// upright's thickness puts it, so the edit is that dimension's: drawn 15, moved out by `shift`.
// The foot's top and the upright's top are the section's edges beside it, so they follow it.
unit mm
use std
use std.solids

thick := 15mm   // the upright as it was drawn
shift := 10mm   // the edit: the riser moved out, along +x

in std.front {
  a := point
  b := point hint((40, 0))
  c := point hint((40, 10))
  d := point hint((25, 10))
  e := point hint((25, 30))
  f := point hint((0, 30))
  horizontal (bottom := line(a, b)) -> vertical (toe := line(b, c)) ->
  horizontal (tread := line(c, d)) -> vertical (riser := line(d, e)) ->
  horizontal (top := line(e, f)) -> vertical (back := line(f, a)) -> close
  fix((0, 0)) a
  a distance(40) b
  b distance(10) c
  a distance(30) f
  f distance(thick + shift) e
}

part := solid(std.solids.Prism(face(bottom, toe, tread, riser, top, back), from: -20mm, to: 0mm))
