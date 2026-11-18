// A C-shaped bracket with a hole drilled through its upper arm only: the C's outline drawn in the
// front plane (x, z) as a chain of runs and the prism over it from y = −10 to 10; the hole's circle
// of radius 4 drawn on a plane lying on the upper arm's top, and the drill its prism down through
// the arm's thickness, so it comes out into the gap and the lower arm is left whole. (The drill
// written as "down to the arm's underside", its prism's points on the material side of
// `bracket.upper_under`, is read only point by point and builds no exact solid, so its depth is
// the arm's.)
unit mm
use std
use std.solids (cut)

reach := 40mm      // the arms' length
arm := 6mm         // each arm's thickness, and the back's
gap := 18mm        // between the arms
r := 4mm

in std.front {
  a := point
  fix((0mm, 0mm)) a
  outline := step(u: reach) (base := line(a)) -> step(dv: arm) (lower_end := line) ->
             step(u: arm) (lower_top := line) -> step(dv: gap) (inside := line) ->
             step(u: reach) (upper_under := line) -> step(dv: arm) (upper_end := line) ->
             step(u: 0mm) (top := line) -> (back := line(p2: a)) -> close
}
bracket := solid(std.solids.Prism(outline, from: -10mm, to: 10mm))
part := solid(bracket)

// the plane the hole is drawn on, lying on the upper arm's top, its origin over the hole's axis
deck := plane(u: hint(dir: (1, 0, 0)), v: hint(dir: (0, 1, 0))) hint(origin: (28, 0, 30))
deck.origin in boundary(bracket.top.side)
deck tangent(at: deck.origin) boundary(bracket.top.side)
deck.u parallel std.x
deck.v parallel std.y
std.origin distance(28mm, along: std.x) deck.origin
std.origin distance(0mm, along: std.y) deck.origin
in deck {
  hole := circle(center: deck.origin) hint(r: 4)
  radius(r) hole
  hole_f := face(hole)
}
drill := solid(std.solids.Prism(hole_f, from: -arm, to: 0mm))
drill cut part
