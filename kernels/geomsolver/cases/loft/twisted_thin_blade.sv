// A thin blade twisted through a whole turn: the rectangle 40 × 0.4 about the origin, its long
// sides along x, drawn once in the top plane and carried up the z axis from z = 0 to z = 60 into
// the plane square to it at each station by the turn of 360° · s about the axis, right-handed.
// Each corner runs along a helix, and each wall is swept by a straight side of the rectangle
// between two of them, a twisted ruled face.
unit mm
use std
use std.shapes
use std.stations

// the section at station `s`: the drawn rectangle carried by the turn taking its frame `o`, `x` to
// the station's origin and to `Twisted`'s point, drawn in at the station
component Blade(g: line, prof: face, o: point, x: point, s: Scalar) {
  st := std.stations.OnLine(g, up: std.y, s: s)
  tw := std.stations.Twisted(st.n, twist: 360deg, reach: 10mm, s: s)
  sec := prof mapped(o -> st.n.origin, x -> tw.t) in st.n
}

in std.top {
  o := point
  fix((0, 0)) o
  x := point
  fix((10, 0)) x
  outline := std.shapes.CenteredRectangle(o, w: 40mm, h: 0.4mm)
}
blade := face(outline.loop)

// the axis, drawn in the front plane (x, z)
in std.front {
  base := point
  fix((0, 0)) base
  tip := point
  fix((0, 60)) tip
  construction centerline spine := line(base, tip)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Blade(spine, blade, o, x).sec at s })
