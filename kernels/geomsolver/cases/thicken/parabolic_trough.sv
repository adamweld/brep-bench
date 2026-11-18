// A parabolic trough thickened by 1 on its concave side. The sheet is the B-spline surface of
// degree 2 in u and 1 in v, control points (−4, y, 2), (0, y, −2), (4, y, 2) for y = 0 and y = 10:
// the developable panel between its two parabolic edges, each that quadratic Bézier drawn as the
// cubic it is (its inner control points two thirds of the way from its ends to the middle one), in
// the planes y = 0 and y = 10, its rulings straight across along y. The thickened sheet is its band
// 1 thick on its outside, the side its normal a′ × (b − a) points to: up, toward the focus, with
// the edge on y = 0 run along +x given first.
unit mm
use std
use std.panels

// a quadratic Bézier from `a` through the control point `c` to `b`, as its cubic
component Parabola(a: point, c: point, b: point) {
  k1 := point
  k2 := point
  k1 coincident(t == 2 / 3) line(a, c)
  k2 coincident(t == 2 / 3) line(b, c)
  edge := spline(a, k1, k2, b)
}

in std.front {
  a0 := point
  c0 := point
  b0 := point
  fix((-4mm, 2mm)) a0
  fix((0mm, -2mm)) c0
  fix((4mm, 2mm)) b0
}
near_edge := Parabola(a0, c0, b0) in std.front        // on y = 0
far_plane := plane
fix(origin == (0mm, 10mm, 0mm)) far_plane
fix(dir == (1, 0, 0)) far_plane.u
fix(dir == (0, 0, 1)) far_plane.v
in far_plane {
  a1 := point
  c1 := point
  b1 := point
  fix((-4mm, 2mm)) a1
  fix((0mm, -2mm)) c1
  fix((4mm, 2mm)) b1
}
far_edge := Parabola(a1, c1, b1) in far_plane         // on y = 10

sheet := std.panels.Developable(near_edge.edge, far_edge.edge)
part := solid({ q | q distance(<= 1mm, side: outside) sheet })
