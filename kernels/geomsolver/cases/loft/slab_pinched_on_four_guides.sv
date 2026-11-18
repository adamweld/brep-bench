// A slab pinched thin on four Bézier guides: each guide a cubic Bézier curve in space, the one
// through the corner with x and y positive on the control points (12, 3.2, 0), (18, -2.8, 8),
// (15, 0.2, 19) and (3, 12.2, 24), the other three its mirror images. The section at the fraction
// `s` of the way up the z axis from z = 0 to z = 24 is the rectangle in the plane at that height
// whose four corners lie on the four guides; each wall is swept by a straight side of it between
// two guides.
unit mm
use std
use std.stations

// a guide: the Bézier on the given control points, mirrored by the signs `sx` and `sy`
component Guide(sx: Scalar, sy: Scalar) {
  k0 := point
  k1 := point
  k2 := point
  k3 := point
  fix((sx * 12mm, sy * 3.2mm, 0mm)) k0
  fix((sx * 18mm, sy * -2.8mm, 8mm)) k1
  fix((sx * 15mm, sy * 0.2mm, 19mm)) k2
  fix((sx * 3mm, sy * 12.2mm, 24mm)) k3
  g := spline(k0, k1, k2, k3)
}

// the section at station `s`: in the level plane there, a corner on each guide
component Slab(axis: line, a: path, b: path, c: path, d: path, s: Scalar) {
  st := std.stations.OnLine(axis, up: std.y, s: s)
  qa := point in st.n
  qb := point in st.n
  qc := point in st.n
  qd := point in st.n
  qa coincident a
  qb coincident b
  qc coincident c
  qd coincident d
  f := face(qa, qb, qc, qd, -> close)
}

ne := Guide(sx: 1, sy: 1)
nw := Guide(sx: -1, sy: 1)
sw := Guide(sx: -1, sy: -1)
se := Guide(sx: 1, sy: -1)

// the axis, drawn in the front plane (x, z)
in std.front {
  base := point
  fix((0, 0)) base
  tip := point
  fix((0, 24)) tip
  construction centerline spine := line(base, tip)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Slab(spine, ne.g, nw.g, sw.g, se.g).f at s })
