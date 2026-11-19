// An ellipsoid halved: the ellipsoid with semi-axes 6 along x, 4 along y and 3 along z, less a
// block standing below the top plane and reaching past it all round. The ellipsoid is the ellipses
// it is cut into level, run by the latitude θ = 180° s - 90°: the section at height z = 3 sin θ on
// the z axis is the ellipse about the axis with semi-axes 6 cos θ along x and 4 cos θ along y,
// closing to a point at each pole. Each section is four weighted cubic splines, each a quarter of
// the ellipse exactly: the quarter circle's rational cubic (its inner control points (2 - sqrt(2))
// of the way along the end tangents, weighing (1 + sqrt(2)) / 3) stretched to the semi-axes.
unit mm
use std
use std.place (coords)
use std.shapes
use std.solids
use std.solids (cut)

// a level plane on the z axis, `z` above `o`, its axes along x and y
component Level(o: point, z: Length) {
  n := plane(u: hint(dir: (1, 0, 0)), v: hint(dir: (0, 1, 0)))
  n.u parallel(sense: same) std.x
  n.v parallel(sense: same) std.y
  n.origin coincident std.z
  o distance(z, along: std.z) n.origin
}

c := 2 - sqrt(2)
heavy := (1 + sqrt(2)) / 3

// the level plane at height 3 sin θ, and the ellipse drawn in it
component Slice(o: point, s: Scalar) {
  st := Level(o, z: -3mm * cos(180deg * s))
  a := 6mm * sin(180deg * s)
  b := 4mm * sin(180deg * s)
  in st.n {
    e0 := point
    e1 := point
    e2 := point
    e3 := point
    e0 coords(du: a, dv: 0mm) st.n
    e1 coords(du: 0mm, dv: b) st.n
    e2 coords(du: -a, dv: 0mm) st.n
    e3 coords(du: 0mm, dv: -b) st.n
    m0 := point
    m1 := point
    n0 := point
    n1 := point
    m2 := point
    m3 := point
    n2 := point
    n3 := point
    m0 coords(du: a, dv: c * b) st.n
    m1 coords(du: c * a, dv: b) st.n
    n0 coords(du: -c * a, dv: b) st.n
    n1 coords(du: -a, dv: c * b) st.n
    m2 coords(du: -a, dv: -c * b) st.n
    m3 coords(du: -c * a, dv: -b) st.n
    n2 coords(du: c * a, dv: -b) st.n
    n3 coords(du: a, dv: -c * b) st.n
    q0 := spline(e0, m0, m1, e1) weights [1, heavy, heavy, 1]
    q1 := spline(e1, n0, n1, e2) weights [1, heavy, heavy, 1]
    q2 := spline(e2, m2, m3, e3) weights [1, heavy, heavy, 1]
    q3 := spline(e3, n2, n3, e0) weights [1, heavy, heavy, 1]
  }
  f := face(q0, q1, q2, q3)
}

in std.top {
  o := point
  fix((0, 0)) o
  floor := std.shapes.CenteredRectangle(o, w: 20mm, h: 20mm)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Slice(o).f at s })
below := solid(std.solids.Prism(face(floor.loop), from: -10mm, to: 0mm))
below cut part
