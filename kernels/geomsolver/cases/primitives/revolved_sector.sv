// The rectangle x from 6 to 10, z from 0 to 4, drawn in the front plane (x, z), turned a quarter
// turn about the z axis: right-handed, from +x toward +y.
unit mm
use std
use std.solids

in std.front {
  a := point hint((6, 0))
  b := point hint((10, 0))
  c := point hint((10, 4))
  d := point hint((6, 4))
  section := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((6, 0)) a
  a distance(4) b
  b distance(4) c
}

part := solid(std.solids.Revolution(section, about: std.z, to: 90deg))
