// A square tapering through a half turn: the square of side 30 at the foot, drawn once in the top
// plane, carried up the z axis from z = 0 to z = 40 into the plane square to it at each station
// by the similarity that turns it 180° · s about the axis, right-handed, and scales it by
// 1 - 2s/3, so it ends a square of side 10. Each corner runs along a conical spiral, and each wall
// is swept by a straight side of the square between two of them, a twisted ruled face.
unit mm
use std
use std.shapes
use std.stations

// the section at station `s`: the drawn square carried by the similarity taking its frame `o`,
// `x` to the station's origin and to `Twisted`'s point, turned and drawn in at the station
component Taper(g: line, prof: face, o: point, x: point, s: Scalar) {
  st := std.stations.OnLine(g, up: std.y, s: s)
  tw := std.stations.Twisted(st.n, twist: 180deg, reach: 10mm * (1 - 2 * s / 3), s: s)
  sec := prof mapped(o -> st.n.origin, x -> tw.t) in st.n
}

in std.top {
  o := point
  fix((0, 0)) o
  x := point
  fix((10, 0)) x
  foot := std.shapes.CenteredRectangle(o, w: 30mm, h: 30mm)
}
square := face(foot.loop)

// the axis, drawn in the front plane (x, z)
in std.front {
  base := point
  fix((0, 0)) base
  tip := point
  fix((0, 40)) tip
  construction centerline spine := line(base, tip)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Taper(spine, square, o, x).sec at s })
