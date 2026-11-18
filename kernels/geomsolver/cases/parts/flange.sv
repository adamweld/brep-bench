// A bored mounting flange: one stepped half-section, the plate, the hub and the bore, turned a
// whole turn about the z axis; then six bolt holes cut through the plate on a circle of radius 24,
// drawn on the plate's top and cut by the library's bolt pattern.
unit mm
use std
use std.solids
use std.place (horizontal)
use hardware

outer_radius := 32mm
bore_radius := 8mm
plate_depth := 6mm
hub_radius := 15mm
hub_height := 10mm
bolt_circle := 24mm
bolt_radius := 3mm
bolts := 6

// each run one number, as the part is dimensioned: a radius from the axis or a height
in std.front {
  top := point hint((0mm, plate_depth + hub_height))
  construction centerline spindle := vertical line(std.origin, top)
  profile := step(u: outer_radius) (ab := line) -> step(v: plate_depth) (bc := line) ->
             step(u: hub_radius) (cd := line) -> step(dv: hub_height) (de := line) ->
             step(u: bore_radius) (ef := line) -> step(v: 0mm) (fa := line) -> close
  deck := point hint((0mm, plate_depth))     // where the axis meets the plate's top
}
top horizontal ef.p2
deck coincident spindle
deck horizontal bc.p2
part := solid(std.solids.Revolution(profile, about: spindle))

// the bolt circle, drawn on the plate's top: a plane over a reference out from the axis
in std.front {
  construction centerline reference := line(deck, bc.p2)
}
square := axis hint(dir: (0, 1, 0))
square parallel std.y
plate_top := plane(u: reference, v: square)
deck coincident plate_top.origin
in plate_top {
  construction centerline across := horizontal line(plate_top.origin, hint((10mm, 0mm)))
  length(10mm) across
}
pattern := hardware.BoltPattern(part, plate_top.origin, across, n: bolts, pitch_r: bolt_circle,
                                hole_r: bolt_radius, phase: 0deg) in plate_top
