// A circle of radius 2 swept a quarter turn along the arc of radius 10 about the z axis, from
// (10, 0, 0) to (0, 10, 0): at each station the section stands square to the path, and the ends
// are the sections themselves.
unit mm
use std
use std.place (vertical)
use std.stations

// the section at station `s`: square to the path there, its `v` along z
component Tube(g: arc, up: axis, s: Scalar) {
  st := std.stations.OnArc(g, up: up, s: s)
  rim := circle(center: st.n.origin) hint(r: 2) in st.n
  radius(2mm) rim
  section := face(rim)
}

in std.top {
  o := point
  fix((0, 0)) o
  start := point
  fix((10, 0)) start
  end := point hint((0, 10))
  o vertical end
  construction path := arc(center: o, start: start, end: end)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Tube(path, std.z).section at s })
