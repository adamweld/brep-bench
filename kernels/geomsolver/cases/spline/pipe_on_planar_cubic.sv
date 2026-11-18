// A fat pipe round a cubic Bézier bend: the disk of radius 20 carried along the Bézier on the
// control points (0, 0), (12, -16), (32, -16) and (44, 0) of the top plane, drawn at each station
// in the plane square to the curve there (`std.stations.OnSpline`, its `v` along z); the ends are
// the sections at the curve's ends.
unit mm
use std
use std.stations

component Tube(g: spline, s: Scalar) {
  st := std.stations.OnSpline(g, up: std.z, s: s)
  in st.n {
    rim := circle(center: st.n.origin)
    radius(20mm) rim
  }
  f := face(rim)
}

in std.top {
  k0 := point
  k1 := point
  k2 := point
  k3 := point
  fix((0, 0)) k0
  fix((12, -16)) k1
  fix((32, -16)) k2
  fix((44, 0)) k3
  construction path := spline(k0, k1, k2, k3)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Tube(path).f at s })
