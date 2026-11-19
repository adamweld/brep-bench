// Two slabs united along flat B-spline faces in the plane z = 4. Solvent has no surface patch, so
// each slab is the face family of its sections across x, its patch's face swept by the patch's
// own curve across y: each column of the patch's net is a Bézier along x, and at u along the
// columns their points stand the control points of the patch's curve across y there (the tensor
// product). The first slab, the box from (0, 0, 0) to (10, 10, 4), has the bicubic patch on
// x = 0, 2, 7, 10 and y = 0, 4, 5, 10 for its top; the second, the box from (5, 3, 4) to
// (15, 13, 7), the biquadratic patch on x = 5, 12, 15 and y = 3, 4, 13 for its bottom. A spline
// here is a cubic, so each quadratic is raised to one exactly: its inner control points two thirds
// of the way from each end toward the middle one.
unit mm
use std
use std.solids (union)

// a cubic Bézier column of the first patch, over y = `y` at height 4
component Column(y: Length) {
  k0 := point
  k1 := point
  k2 := point
  k3 := point
  fix((0mm, y, 4mm)) k0
  fix((2mm, y, 4mm)) k1
  fix((7mm, y, 4mm)) k2
  fix((10mm, y, 4mm)) k3
  g := spline(k0, k1, k2, k3)
}

// a quadratic Bézier column of the second patch, over y = `y` at height 4, raised to a cubic
component RaisedColumn(y: Length) {
  a := point
  m := point
  b := point
  fix((5mm, y, 4mm)) a
  fix((12mm, y, 4mm)) m
  fix((15mm, y, 4mm)) b
  near := line(a, m)
  far := line(b, m)
  k1 := point
  k2 := point
  k1 coincident(t == 2 / 3) near
  k2 coincident(t == 2 / 3) far
  g := spline(a, k1, k2, b)
}

// the plane across x through the point at `u` of the column `c`, its `u` along y and its `v` up
component Across(c: path, u: Scalar) {
  m := point
  m coincident(t == u) c
  n := plane(u: hint(dir: (0, 1, 0)), v: hint(dir: (0, 0, 1)))
  n.u parallel(sense: same) std.y
  n.v parallel(sense: same) std.z
  n.origin coincident m
}

// the first slab's section at `u`: the floor z = 0 below the patch's cubic across y on the four
// columns' crossings
component Lower(c0: path, c1: path, c2: path, c3: path, u: Scalar) {
  st := Across(c0, u: u)
  in st.n {
    q0 := point hint((0, 0))
    q1 := point hint((4, 0))
    q2 := point hint((5, 0))
    q3 := point hint((10, 0))
    f0 := point hint((0, -4))
    f3 := point hint((10, -4))
    f0 distance(0mm, along: u) q0
    f3 distance(0mm, along: u) q3
    floor := line(f0, f3)
    east := line(f3, q3)
    top := spline(q3, q2, q1, q0)
    west := line(q0, f0)
  }
  f0 coincident std.top
  f3 coincident std.top
  q0 coincident c0
  q1 coincident c1
  q2 coincident c2
  q3 coincident c3
  f := face(floor, east, top, west)
}

// the second slab's section at `u`: the patch's quadratic across y on the three columns'
// crossings, raised to a cubic, below the lid z = 7
component Upper(c0: path, c1: path, c2: path, lid: plane, u: Scalar) {
  st := Across(c0, u: u)
  in st.n {
    q0 := point hint((3, 0))
    m := point hint((4, 0))
    q2 := point hint((13, 0))
    construction near := line(q0, m)
    construction far := line(q2, m)
    k1 := point
    k2 := point
    k1 coincident(t == 2 / 3) near
    k2 coincident(t == 2 / 3) far
    t0 := point hint((3, 3))
    t2 := point hint((13, 3))
    t0 distance(0mm, along: u) q0
    t2 distance(0mm, along: u) q2
    bottom := spline(q0, k1, k2, q2)
    east := line(q2, t2)
    top := line(t2, t0)
    west := line(t0, q0)
  }
  t0 coincident lid
  t2 coincident lid
  q0 coincident c0
  m coincident c1
  q2 coincident c2
  f := face(bottom, east, top, west)
}

a0 := Column(y: 0mm)
a1 := Column(y: 4mm)
a2 := Column(y: 5mm)
a3 := Column(y: 10mm)
b0 := RaisedColumn(y: 3mm)
b1 := RaisedColumn(y: 4mm)
b2 := RaisedColumn(y: 13mm)

lid := plane
fix(origin == (0, 0, 7)) lid
fix(dir == (1, 0, 0)) lid.u
fix(dir == (0, 1, 0)) lid.v

param u: Scalar over (0, 1)
part := solid({ p | some u: p in Lower(a0.g, a1.g, a2.g, a3.g).f at u })
second := solid({ p | some u: p in Upper(b0.g, b1.g, b2.g, lid).f at u })
second union part
