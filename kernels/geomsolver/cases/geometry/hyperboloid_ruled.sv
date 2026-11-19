// A hyperboloid of one sheet as the surface ruled between two circles of radius 5 about the z
// axis, at z = -5 and z = 5, each point at angle θ on the lower joined by a straight line to the
// point at θ + 90° on the upper. Turning about the axis carries each ruling into the next, so the
// side is everything one ruling, the line from (5, 0, -5) to (0, 5, 5), sweeps turning: the section
// at the fraction `s` of the way up the axis is the level circle about the axis through the point
// where that ruling crosses its plane. The ends are the two disks.
unit mm
use std
use std.stations

component Slice(axis: line, ruling: line, s: Scalar) {
  st := std.stations.OnLine(axis, up: std.y, s: s)
  q := point in st.n
  q coincident ruling
  in st.n {
    rim := circle(center: st.n.origin)
    q coincident rim
  }
  f := face(rim)
}

low := point
high := point
fix((5, 0, -5)) low
fix((0, 5, 5)) high
ruling := line(low, high)

// the axis, drawn in the front plane (x, z)
in std.front {
  south := point
  fix((0, -5)) south
  north := point
  fix((0, 5)) north
  construction centerline spine := line(south, north)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Slice(spine, ruling).f at s })
