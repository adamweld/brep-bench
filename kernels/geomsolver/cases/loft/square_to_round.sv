// A square-to-round transition, as one section that runs up the z axis from z = 0 to z = 30: at
// the fraction `s` of the way up, the square 40 across at the foot and the circle of radius 15 at
// the top mixed, (1 - s) of one and s of the other — a rounded square, its four flats 40 (1 - s)
// long and its corners quarter arcs of radius 15 s about the points 20 (1 - s) out along each
// diagonal. Each flat runs between two corners' arcs, tangent to them, so it sweeps a flat
// triangle from a side of the square to the point of the circle nearest it, and each corner's arc
// the oblique cone from a corner of the square to a quarter of the circle. At s = 0 the arcs are
// the square's corners and at s = 1 the flats are the circle's four points.
unit mm
use std
use std.place (coords)
use std.stations

component Transition(g: line, up: axis, s: Scalar) {
  st := std.stations.OnLine(g, up: up, s: s)
  in st.n {
    h := 20mm * (1 - s)
    r := 15mm * s
    c0 := point
    c1 := point
    c2 := point
    c3 := point
    c0 coords(du: h, dv: h) st.n
    c1 coords(du: -h, dv: h) st.n
    c2 coords(du: -h, dv: -h) st.n
    c3 coords(du: h, dv: -h) st.n
    // the flats' ends, seeded where the rounded square has them
    t1 := point hint((h, h + r))
    t2 := point hint((-h, h + r))
    l1 := point hint((-h - r, h))
    l2 := point hint((-h - r, -h))
    b1 := point hint((-h, -h - r))
    b2 := point hint((h, -h - r))
    r1 := point hint((h + r, -h))
    r2 := point hint((h + r, h))
    outline := (k0 := arc(center: c0) hint(r: r)) -> tangent (top := line(t1, t2)) ->
      tangent (k1 := arc(center: c1) hint(r: r)) -> tangent (left := line(l1, l2)) ->
      tangent (k2 := arc(center: c2) hint(r: r)) -> tangent (bottom := line(b1, b2)) ->
      tangent (k3 := arc(center: c3) hint(r: r)) -> tangent (right := line(r1, r2)) ->
      tangent close
    radius(r) k0
    k0 equal k1
    k1 equal k2
    k2 equal k3
  }
  f := face(outline)
}

// the axis, drawn in the front plane (x, z): from the square's centre to the circle's
in std.front {
  o := point
  fix((0, 0)) o
  e := point
  fix((0, 30)) e
  construction centerline spine := line(o, e)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Transition(spine, std.x).f at s })
