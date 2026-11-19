// A helical pipe: a circle of radius 1 carried along the helix of radius 10 about the z axis that
// rises 5 a turn, for three turns from (10, 0, 0), right-handed. At each station the circle is
// drawn in the plane square to the helix (`std.stations.OnHelix`), and its two ends are the
// sections there.
unit mm
use std
use std.stations

component Coil(s: Scalar) {
  st := std.stations.OnHelix(std.top, r: 10mm, pitch: 5mm, turns: 3, s: s)
  in st.n {
    wire := circle(center: st.n.origin)
    radius(1mm) wire
  }
  f := face(wire)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Coil().f at s })
