// A block under a bicubic Bézier dome. Solvent has no surface patch, so the block is the face
// family of the patch's sections across x: each column j of the net, its four control points
// P_ij = (20i/3, 20j/3, z_ij) for i = 0 … 3, is a cubic Bézier along x, and where the plane
// x = 20s crosses the four columns stand the control points of the patch's own cubic across y
// there — the tensor product, S(u, v) = Σ_j B_j(v) (Σ_i B_i(u) P_ij). The section at s is the
// region of that plane between the floor z = 0, the upright sides y = 0 and y = 20, and that
// cubic.
unit mm
use std
use std.place (coords)
use std.stations

// column j of the net: the control points over y = `y`, at the heights `z0` … `z3` along x
component Column(y: Length, z0: Length, z1: Length, z2: Length, z3: Length) {
  k0 := point
  k1 := point
  k2 := point
  k3 := point
  fix((0mm, y, z0)) k0
  fix((20mm / 3, y, z1)) k1
  fix((40mm / 3, y, z2)) k2
  fix((20mm, y, z3)) k3
  g := spline(k0, k1, k2, k3)
}

// the section where the plane x = 20s crosses the columns: the floor, the two sides and the
// patch's cubic across y on the four crossings
component Slice(along: line, c0: path, c1: path, c2: path, c3: path, s: Scalar) {
  st := std.stations.OnLine(along, up: std.z, s: s)
  in st.n {
    q0 := point hint((0, 5))
    q1 := point hint((6.67, 10))
    q2 := point hint((13.3, 10))
    q3 := point hint((20, 6))
    f0 := point
    f3 := point
    f0 coords(du: 0mm, dv: 0mm) st.n
    f3 coords(du: 20mm, dv: 0mm) st.n
    floor := line(f0, f3)
    east := line(f3, q3)
    top := spline(q3, q2, q1, q0)
    west := line(q0, f0)
  }
  q0 coincident c0
  q1 coincident c1
  q2 coincident c2
  q3 coincident c3
  f := face(floor, east, top, west)
}

col0 := Column(y: 0mm, z0: 4mm, z1: 6mm, z2: 5mm, z3: 3mm)
col1 := Column(y: 20mm / 3, z0: 6mm, z1: 14mm, z2: 13mm, z3: 6mm)
col2 := Column(y: 40mm / 3, z0: 7mm, z1: 12mm, z2: 16mm, z3: 8mm)
col3 := Column(y: 20mm, z0: 5mm, z1: 7mm, z2: 8mm, z3: 6mm)

// the run along x under the block
in std.top {
  o := point
  fix((0, 0)) o
  e := point
  fix((20, 0)) e
  construction along := line(o, e)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Slice(along, col0.g, col1.g, col2.g, col3.g).f at s })
