// A pipe along a twisted cubic Bézier: the disk of radius 8 carried along the Bézier on the control
// points (0, 0, 0), (12, 16, 0), (26, 8, 0) and (24, 0, 16), a curve in space, drawn at each
// station in the plane square to the curve there (`std.stations.On`, its frame turning only as the
// curve makes it); the ends are the sections at the curve's ends.
unit mm
use std
use std.stations

component Tube(g: path, s: Scalar) {
  st := std.stations.On(g, up: std.z, s: s)
  in st.n {
    rim := circle(center: st.n.origin)
    radius(8mm) rim
  }
  f := face(rim)
}

k0 := point
k1 := point
k2 := point
k3 := point
fix((0, 0, 0)) k0
fix((12, 16, 0)) k1
fix((26, 8, 0)) k2
fix((24, 0, 16)) k3
path := spline(k0, k1, k2, k3)

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Tube(path).f at s })
