// Half a torus: the tube's circle, radius 2, drawn in the front plane (x, z) 10 out along x,
// turned a whole turn about the z axis; and everything where x < 0 cut from it, by a cutter
// through the torus along z over a rectangle in the top plane whose edge `seam` lies on the
// y axis and which reaches past the torus on every other side (13 back along −x, 13 either way
// along y).
unit mm
use std
use std.solids
use std.solids (cut)
use std.place (right_of)

in std.front {
  c := point hint((10, 0))
  c right_of(d: 10mm) std.origin
  tube := circle(center: c) hint(r: 2)
  radius(2mm) tube
}

in std.top {
  shadow := step(u: 0mm, v: -13mm) (south := line) -> step(v: 13mm) (seam := line) ->
            step(u: -13mm) (north := line) -> step(v: -13mm) (west := line) -> close
}

behind := solid(std.solids.Through(shadow))
part := solid(std.solids.Revolution(face(tube), about: std.z))
behind cut part
