// A square bar bent along a cubic Bézier: the square 20 × 20 centred on the Bézier on the control
// points (0, 0), (16, 12), (18, 26) and (12, 34) of the top plane, drawn at each station in the
// plane square to the curve there (`std.stations.OnSpline`, its `v` along z, so two sides stay
// level); the ends are the sections at the curve's ends.
unit mm
use std
use std.shapes
use std.stations

component Bar(g: spline, s: Scalar) {
  st := std.stations.OnSpline(g, up: std.z, s: s)
  sq := std.shapes.CenteredRectangle(st.n.origin, w: 20mm, h: 20mm) in st.n
  f := face(sq.loop)
}

in std.top {
  k0 := point
  k1 := point
  k2 := point
  k3 := point
  fix((0, 0)) k0
  fix((16, 12)) k1
  fix((18, 26)) k2
  fix((12, 34)) k3
  construction path := spline(k0, k1, k2, k3)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Bar(path).f at s })
