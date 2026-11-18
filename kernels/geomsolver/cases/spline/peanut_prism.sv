// A peanut prism: the closed uniform cubic B-spline on twelve control points of the top plane,
// the prism over it from z = 0 to z = 10. Solvent has no periodic spline, so each of the curve's
// twelve spans is drawn as the cubic Bézier it is (Boehm's construction, in relations): on each
// leg of the control polygon the points a third and two thirds along it, and at each control point
// the junction halfway between the two thirds-points either side of it; the span along a leg runs
// from the junction at its start, through its two thirds-points, to the junction at its end.
unit mm
use std
use std.solids

net := [(14mm, 0mm), (12mm, 8mm), (4mm, 6mm), (0mm, -21mm / 8), (-4mm, 6mm), (-12mm, 8mm),
        (-14mm, 0mm), (-12mm, -8mm), (-4mm, -6mm), (0mm, 21mm / 8), (4mm, -6mm), (12mm, -8mm)]

in std.top {
  cycle h in net {
    k := point
    fix((h.x, h.y)) k
    construction leg := line(k, next.k)
    a := point
    b := point
    a coincident(t == 1 / 3) leg
    b coincident(t == 2 / 3) leg
    construction across := line(prev.b, a)
    j := point
    j midpoint across
    span := spline(j, a, b, next.j)
  }
}

part := solid(std.solids.Prism(face(span), from: 0mm, to: 10mm))
