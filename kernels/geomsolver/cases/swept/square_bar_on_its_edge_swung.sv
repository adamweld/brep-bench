// The solid a square bar standing on its edge fills as it is swung a quarter turn about z. The
// bar's section, in the top plane, is the square stood on a corner with its diagonal along x
// from (4, 0) to (8, 0): its sides at 45 degrees, equal, square to one another. The bar is the
// prism over it from z = 0 to 3, and the part is the kernel's sweep of it turned right-handed
// about z through 90 degrees.
unit mm
use std
use std.solids
use std.motion (turns)

in std.top {
  private a := point
  private b := point hint((6, 2))
  private c := point hint((8, 0))
  private d := point hint((6, -2))
  fix((4, 0)) a
  fix((8, 0)) c
  (ab := line(a, b)) -> perpendicular equal (bc := line(b, c)) ->
  (cd := line(c, d)) -> perpendicular equal (da := line(d, a)) -> close
}
construction bar := solid(std.solids.Prism(face(ab, bc, cd, da), from: 0mm, to: 3mm))

swing := frame
swing turns(1) std.z
part := solid({ p | some t in (0deg, 90deg): p in bar at(under: swing) t })
