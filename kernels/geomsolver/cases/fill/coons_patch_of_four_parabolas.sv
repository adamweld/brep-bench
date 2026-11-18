// A block capped by the bilinearly blended Coons patch of four parabolic arcs. Solvent has no fill
// of a loop of curves, so the patch is written as what the blend is, a face family: the section in
// the plane x = 10u runs from the floor up to the arcs on y = 0 and y = 10 (its corners where the
// plane cuts them, at their own parameter u), and its top is the parabola over that chord whose
// middle control point stands off the chord's middle by the bulge of the arc on x = 0 blended
// linearly into the bulge of the arc on x = 10 — the Coons patch's section at u. Each parabola, a
// quadratic Bézier, is drawn as the cubic it is (its inner control points two thirds of the way
// from its ends to its middle control point).
unit mm
use std
use std.stations

// the arcs' heights, from the case: ends at 10, middle control points 14 (y = 0, x = 0) and 6
// (x = 10, y = 10); each end arc's bulge is its middle control point's rise over its chord
base := 10mm
bulge_near := 14mm - base        // the arc on x = 0
bulge_far := 6mm - base          // the arc on x = 10

// a quadratic Bézier from `a` through the control point `c` to `b`, as its cubic
component Parabola(a: point, c: point, b: point) {
  k1 := point
  k2 := point
  k1 coincident(t == 2 / 3) line(a, c)
  k2 coincident(t == 2 / 3) line(b, c)
  arc_ := spline(a, k1, k2, b)
}

in std.front {
  a0 := point
  c0 := point
  b0 := point
  fix((0mm, base)) a0
  fix((5mm, 14mm)) c0
  fix((10mm, base)) b0
}
south := Parabola(a0, c0, b0) in std.front           // the arc on y = 0
north_plane := plane
fix(origin == (0mm, 10mm, 0mm)) north_plane
fix(dir == (1, 0, 0)) north_plane.u
fix(dir == (0, 0, 1)) north_plane.v
in north_plane {
  a1 := point
  c1 := point
  b1 := point
  fix((0mm, base)) a1
  fix((5mm, 6mm)) c1
  fix((10mm, base)) b1
}
north := Parabola(a1, c1, b1) in north_plane         // the arc on y = 10

in std.front {
  g1 := point
  fix((10mm, 0mm)) g1
  construction guide := line(std.origin, g1)          // along x, under the arc on y = 0
}

// the Coons patch's section at u = s, and the block under it
component Section(south: path, north: path, np: plane, g: line, b0: Length, b1: Length, s: Scalar) {
  st := std.stations.OnLine(g, up: std.z, s: s)       // the plane x = 10s: u along y, v along z
  in st.n {
    f0 := point hint((0, 0))
    f1 := point hint((10, 0))
    f0 distance(10mm, along: u) f1
    f0 level(v) f1
    mid := point hint((5, 10))
    top := point hint((5, 10))
    m0 := point hint((3, 10))
    m2 := point hint((7, 10))
  }
  f0 coincident st.n.origin
  q0 := point in st.n, std.front hint((0, 10))
  q2 := point in st.n, np hint((10, 10))
  q0 coincident south                                 // where the plane cuts the arc on y = 0
  q2 coincident north                                 // ... and the arc on y = 10
  in st.n {
    mid midpoint line(q0, q2)
    mid distance(b0 * (1 - s) + b1 * s, along: v) top
    mid level(u) top
    m0 coincident(t == 2 / 3) line(q0, top)
    m2 coincident(t == 2 / 3) line(q2, top)
    cap := spline(q2, m2, m0, q0)
    floor := line(f0, f1)
    east := line(f1, q2)
    west := line(q0, f0)
  }
  f := face(floor, east, cap, west)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Section(south.arc_, north.arc_, north_plane, guide,
                                         b0: bulge_near, b1: bulge_far, s: s).f at s })
