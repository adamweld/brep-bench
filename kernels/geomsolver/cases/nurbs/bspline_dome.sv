// A block under a bicubic B-spline surface with a double knot. Solvent has no surface patch, so
// the block is the face family of the surface's sections across x: each column j of the net, its
// six control points P_ij = (x_i, y_j, z_ij), is a cubic B-spline along x on the knots
// (0, 0, 0, 0, 2/5, 2/5, 1, 1, 1, 1), and at each u their points stand the control points of the
// surface's own cubic B-spline across y there, on the knots
// (0, 0, 0, 0, 1/3, 1, 1, 1, 1) — the tensor product, S(u, v) = Σ_j M_j(v) (Σ_i N_i(u) P_ij). The
// section at u is the region of the plane across x through the columns' points at u between the
// floor z = 0, the upright sides y = 0 and y = 20, and that curve.
unit mm
use std

// column j of the net: the control points over y = `y`, at x = 0, 3, 7, 11, 16 and 20 and the
// heights `z0` … `z5`
component Column(y: Length, z0: Length, z1: Length, z2: Length, z3: Length, z4: Length,
                 z5: Length) {
  k0 := point
  k1 := point
  k2 := point
  k3 := point
  k4 := point
  k5 := point
  fix((0mm, y, z0)) k0
  fix((3mm, y, z1)) k1
  fix((7mm, y, z2)) k2
  fix((11mm, y, z3)) k3
  fix((16mm, y, z4)) k4
  fix((20mm, y, z5)) k5
  g := spline(k0, k1, k2, k3, k4, k5) knots [0, 0, 0, 0, 0.4, 0.4, 1, 1, 1, 1]
}

// the section at `u` along the columns: the plane across x through the first column's point
// there, its `u` along y and its `v` up; the floor, the two sides and the surface's curve across y
// on the five columns' crossings of it
component Slice(c0: path, c1: path, c2: path, c3: path, c4: path, u: Scalar) {
  m := point
  m coincident(t == u) c0
  n := plane(u: hint(dir: (0, 1, 0)), v: hint(dir: (0, 0, 1)))
  n.u parallel(sense: same) std.y
  n.v parallel(sense: same) std.z
  n.origin coincident m
  in n {
    q0 := point hint((0, 0))
    q1 := point hint((5, 4))
    q2 := point hint((10, 7))
    q3 := point hint((15, 4))
    q4 := point hint((20, 0))
    f0 := point hint((0, -5))
    f4 := point hint((20, -5))
    f0 distance(0mm, along: u) q0
    f4 distance(0mm, along: u) q4
    floor := line(f0, f4)
    east := line(f4, q4)
    top := spline(q0, q1, q2, q3, q4) knots [0, 0, 0, 0, 0.3333333333333333, 1, 1, 1, 1]
    west := line(q0, f0)
  }
  f0 coincident std.top
  f4 coincident std.top
  q0 coincident c0
  q1 coincident c1
  q2 coincident c2
  q3 coincident c3
  q4 coincident c4
  f := face(floor, east, top, west)
}

col0 := Column(y: 0mm, z0: 3mm, z1: 5mm, z2: 6mm, z3: 4mm, z4: 5mm, z5: 2mm)
col1 := Column(y: 5mm, z0: 5mm, z1: 9mm, z2: 12mm, z3: 10mm, z4: 8mm, z5: 4mm)
col2 := Column(y: 10mm, z0: 6mm, z1: 11mm, z2: 15mm, z3: 18mm, z4: 9mm, z5: 5mm)
col3 := Column(y: 15mm, z0: 5mm, z1: 8mm, z2: 10mm, z3: 9mm, z4: 7mm, z5: 4mm)
col4 := Column(y: 20mm, z0: 4mm, z1: 5mm, z2: 6mm, z3: 5mm, z4: 4mm, z5: 3mm)

param u: Scalar over (0, 1)
part := solid({ p | some u: p in Slice(col0.g, col1.g, col2.g, col3.g, col4.g).f at u })
