// A turned shaft: its half-section drawn in the front plane (x out from the axis, z along it) as
// a closed chain of runs, each a radius or a height, the inside corner where the shoulder meets
// the neck rounded to radius 4 by a round at that joint; the section turned a whole turn about z.
unit mm
use std
use std.solids

R := 20mm          // the thick part's radius
r := 12mm          // the neck's radius
rho := 4mm         // the shoulder's round
shoulder := 60mm
L := 160mm

in std.front {
  o := point
  fix((0mm, 0mm)) o
  corner := point hint((12, 60))     // the shoulder's corner, which the round takes
  section := step(u: R) (base := line(o)) -> step(v: shoulder) (barrel := line) ->
             (step_face := line(p2: corner)) -> round(r: rho)
             step(v: L) (neck := line(corner)) -> step(u: 0mm) (head := line) ->
             (spine := line(p2: o)) -> close
  o distance(r, along: u) corner
  barrel.p2 level(up) corner
}
part := solid(std.solids.Revolution(section, about: std.z))
