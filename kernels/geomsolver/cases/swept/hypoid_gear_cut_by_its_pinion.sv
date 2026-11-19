// A hypoid gear's tooth space cut by its pinion's tooth. The blank is the turn about the z axis of
// its quadrilateral section in the front plane (x, z). The tooth is the frustum, between the planes
// square to g = (2, 0, 1)/sqrt 5 at 15 sqrt 5 - 12 and 15 sqrt 5 + 2 from the origin, of the
// pyramid joining the origin to a trapezoid at P = 15 sqrt 5 g in the plane square to g, spanned by
// d = (-1, 0, 2)/sqrt 5 and y: |u| <= 2.5 along d and |w| <= pi/2 - u tan 20 along y, a rack tooth
// pi wide at its pitch line with its sides 20 degrees from square; it is the join (the hull) of its
// two end sections. It is turned about the z axis by phi, sin phi = 24/(75 - 5 sqrt 5). The pinion
// turns about the line through (0, 12, 0) along x a radian a radian of the roll, the gear -7/15 of
// a radian about the z axis; seen from the gear, everything the tooth passes through as the roll
// runs from -70 to +70 degrees is cut from the blank.
unit mm
use std
use std.sets
use std.solids
use std.solids (cut)
use std.motion (turns)

in std.front {
  k0 := point
  k1 := point
  k2 := point
  k3 := point
  fix((30.894427, 13.211146)) k0
  fix((27.316718, 20.366563)) k1
  fix((18.372447, 15.894427)) k2
  fix((21.683489, 9.272343)) k3
  section := line(k0, k1) -> line(k1, k2) -> line(k2, k3) -> line(k3, k0) -> close
}
part := solid(std.solids.Revolution(section, about: std.z))

// The frustum is the join of its two ends, each in a plane square to g, spanned by d and y: 12
// nearer the origin than P (the toe) and 2 farther (the back). Each end is the trapezoid at P as
// the pyramid from the origin cuts that plane, scaled by its distance from the origin over P's,
// `heel`.
heel := 15 * sqrt(5)
toe_d := axis hint(dir: (-0.45, 0, 0.89))
fix(dir == (-1 / sqrt(5), 0, 2 / sqrt(5))) toe_d
toe_across := axis hint(dir: (0, 1, 0))
fix(dir == (0, 1, 0)) toe_across
toe := plane(u: toe_d, v: toe_across)
fix(origin == ((heel - 12) * 2 / sqrt(5), 0, (heel - 12) / sqrt(5))) toe
in toe {
  toe0 := point
  toe1 := point
  toe2 := point
  toe3 := point
  fix((2.5 * (heel - 12) / heel, -(pi / 2 - 2.5 * tan(20deg)) * (heel - 12) / heel)) toe0
  fix((2.5 * (heel - 12) / heel, (pi / 2 - 2.5 * tan(20deg)) * (heel - 12) / heel)) toe1
  fix((-2.5 * (heel - 12) / heel, (pi / 2 + 2.5 * tan(20deg)) * (heel - 12) / heel)) toe2
  fix((-2.5 * (heel - 12) / heel, -(pi / 2 + 2.5 * tan(20deg)) * (heel - 12) / heel)) toe3
  toe_end := line(toe0, toe1) -> line(toe1, toe2) -> line(toe2, toe3) -> line(toe3, toe0) ->
             close
  toe_face := face(toe_end)
}
back_d := axis hint(dir: (-0.45, 0, 0.89))
fix(dir == (-1 / sqrt(5), 0, 2 / sqrt(5))) back_d
back_across := axis hint(dir: (0, 1, 0))
fix(dir == (0, 1, 0)) back_across
back := plane(u: back_d, v: back_across)
fix(origin == ((heel + 2) * 2 / sqrt(5), 0, (heel + 2) / sqrt(5))) back
in back {
  back0 := point
  back1 := point
  back2 := point
  back3 := point
  fix((2.5 * (heel + 2) / heel, -(pi / 2 - 2.5 * tan(20deg)) * (heel + 2) / heel)) back0
  fix((2.5 * (heel + 2) / heel, (pi / 2 - 2.5 * tan(20deg)) * (heel + 2) / heel)) back1
  fix((-2.5 * (heel + 2) / heel, (pi / 2 + 2.5 * tan(20deg)) * (heel + 2) / heel)) back2
  fix((-2.5 * (heel + 2) / heel, -(pi / 2 + 2.5 * tan(20deg)) * (heel + 2) / heel)) back3
  back_end := line(back0, back1) -> line(back1, back2) -> line(back2, back3) ->
              line(back3, back0) -> close
  back_face := face(back_end)
}
tooth := solid({ p | some q in toe_face, r in back_face: p in std.sets.Segment(q, r) })

// placed about the gear's axis by phi, where the pitch surfaces touch at the tooth's mean point
placed := tooth at(under: std.z) asin(24 / (75 - 5 * sqrt(5)))

pa := point
fix((0, 12, 0)) pa
pb := point
fix((1, 12, 0)) pb
pinion_axis := line(pa, pb)
pinion := frame
pinion turns(1) pinion_axis
gear := frame
gear turns(-7 / 15) std.z
space := solid({ p | some t in (-70deg, 70deg): p in placed at(under: pinion, seen_from: gear) t })
space cut part
