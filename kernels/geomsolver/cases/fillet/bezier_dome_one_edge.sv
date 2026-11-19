// The block under a bicubic Bézier dome, its top edge on the side y = 0 rounded to radius 1.
// The patch is stated as its curves of constant u: at each station s along x (x = 20s, s = u),
// the section square to x is the region from the floor up to the cubic Bézier curve in y whose
// control points stand at y = 20j/3 and at heights Σᵢ Bᵢ(s) zᵢⱼ — the patch's row of heights
// blended by the cubic Bernstein polynomials in s. Swept over s from 0 to 1, the tops of the
// sections are the patch itself, the sides the planes y = 0 and y = 20, the ends the planes
// x = 0 and x = 20 and the floor z = 0. The edge where the top meets the side swept from `west`
// (y = 0) is rounded to radius 1.
unit mm
use std
use std.stations
use std.place (coords)
use std.rounds (round)

w := 20mm   // the square's side
// the heights zᵢⱼ, a row a group: row i at x = 20i/3, column j at y = 20j/3
z0 := {j0: 4mm, j1: 6mm, j2: 7mm, j3: 5mm}
z1 := {j0: 6mm, j1: 14mm, j2: 12mm, j3: 7mm}
z2 := {j0: 5mm, j1: 13mm, j2: 16mm, j3: 8mm}
z3 := {j0: 3mm, j1: 6mm, j2: 8mm, j3: 6mm}

// the section at station `s`, drawn in the plane square to the guide there: `u` along y, `v` up z
component Section(g: line, s: Scalar) {
  st := std.stations.OnLine(g, up: std.z, s: s)
  // the cubic Bernstein polynomials at s
  b0 := (1 - s)^3
  b1 := 3 * s * (1 - s)^2
  b2 := 3 * s^2 * (1 - s)
  b3 := s^3
  in st.n {
    q0 := point hint((0, 0))
    q0 coords(du: 0mm, dv: 0mm) st.n
    q3 := point hint((20, 0))
    q3 coords(du: w, dv: 0mm) st.n
    k0 := point hint((0, 6))
    k0 coords(du: 0mm, dv: b0 * z0.j0 + b1 * z1.j0 + b2 * z2.j0 + b3 * z3.j0) st.n
    k1 := point hint((6.7, 11))
    k1 coords(du: w / 3, dv: b0 * z0.j1 + b1 * z1.j1 + b2 * z2.j1 + b3 * z3.j1) st.n
    k2 := point hint((13.3, 11))
    k2 coords(du: 2 * w / 3, dv: b0 * z0.j2 + b1 * z1.j2 + b2 * z2.j2 + b3 * z3.j2) st.n
    k3 := point hint((20, 7))
    k3 coords(du: w, dv: b0 * z0.j3 + b1 * z1.j3 + b2 * z2.j3 + b3 * z3.j3) st.n
    floor := line(q0, q3)
    east := line(q3, k3)
    top := spline(k0, k1, k2, k3)
    west := line(k0, q0)
    f := face(floor, east, top, west)
  }
}

in std.top {
  a := point
  fix((0, 0)) a
  b := point hint((20, 0))
  construction centerline guide := line(a, b)
  horizontal guide
  length(w) guide
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Section(guide, s).f at s })
part.west round(r: 1mm) part.top
