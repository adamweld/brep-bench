// A cylinder of radius 10 about the z axis from z = 0 to z = 20, parted at its circle at z = 10 and
// each half drafted 3° away from the parting line: the circle drawn on the plane held at z = 10,
// square to the axis, and the drafted prism over it from 10 below to 10 above. Solvent's draft
// leans the sides away from the face's own plane on both sides — the upper half pulled along +z,
// the lower along -z — so each half is a cone through the parting circle, narrowing toward its end.
unit mm
use std
use std.solids

parting := plane
fix(origin == (0mm, 0mm, 10mm)) parting
fix(dir == (1, 0, 0)) parting.u
fix(dir == (0, 1, 0)) parting.v

in parting {
  line_of_parting := circle(center: parting.origin) hint(r: 10)
  radius(10mm) line_of_parting
}

part := solid(std.solids.Drafted(face(line_of_parting), from: -10mm, to: 10mm, draft: 3deg))
