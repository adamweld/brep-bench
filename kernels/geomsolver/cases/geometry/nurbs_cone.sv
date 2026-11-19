// A cone bounded by a NURBS side with a pole at its apex. The side is rational quadratic round the
// axis and linear up it: its row at height 10 s is the base circle's nine-point rational quadratic
// net scaled by 1 - s, so the side is the face family of those circles, from the base (whose disk
// is the family's start) to the apex (its end, where the row collapses to a point). Solvent has no
// surface patch and its splines are cubics, so each quarter of the circle, a rational quadratic on
// three points of the net weighing 1, √2/2, 1, is raised to a cubic exactly: its inner control
// points the fraction 2w / (1 + 2w) = 2 - √2 of the way from each end toward the middle point,
// each weighing (1 + 2w) / 3, w = √2/2.
unit mm
use std
use std.place (coords)
use std.stations

heavy := (1 + sqrt(2)) / 3
inner := 2 - sqrt(2)

// a quarter of the circle: the rational quadratic on `a`, `m`, `b`, raised to a cubic
component Quarter(a: point, m: point, b: point) {
  in m.plane {
    construction near := line(a, m)
    construction far := line(b, m)
    k1 := point
    k2 := point
    k1 coincident(t == inner) near
    k2 coincident(t == inner) far
    g := spline(a, k1, k2, b) weights [1, heavy, heavy, 1]
  }
}

// the row at height 10 s: the net 5 (1, 0), 5 (1, 1), 5 (0, 1), … scaled by 1 - s
component Row(spine: line, s: Scalar) {
  st := std.stations.OnLine(spine, up: std.y, s: s)
  r := 5mm * (1 - s)
  in st.n {
    c0 := point
    c1 := point
    c2 := point
    c3 := point
    c4 := point
    c5 := point
    c6 := point
    c7 := point
    c0 coords(du: r, dv: 0mm) st.n
    c1 coords(du: r, dv: r) st.n
    c2 coords(du: 0mm, dv: r) st.n
    c3 coords(du: -r, dv: r) st.n
    c4 coords(du: -r, dv: 0mm) st.n
    c5 coords(du: -r, dv: -r) st.n
    c6 coords(du: 0mm, dv: -r) st.n
    c7 coords(du: r, dv: -r) st.n
  }
  q0 := Quarter(c0, c1, c2)
  q1 := Quarter(c2, c3, c4)
  q2 := Quarter(c4, c5, c6)
  q3 := Quarter(c6, c7, c0)
  f := face(q0.g, q1.g, q2.g, q3.g)
}

// the axis, drawn in the front plane (x, z), from the base's centre to the apex
in std.front {
  o := point
  fix((0, 0)) o
  apex := point
  fix((0, 10)) apex
  construction centerline spine := line(o, apex)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Row(spine).f at s })
