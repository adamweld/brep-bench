// A hollow square duct elbow: one hollow square section, 18 across with a wall 2 thick, carried a
// quarter turn round the arc of radius 30 about the z axis, from (30, 0, 0) to (0, 30, 0); at each
// fraction of the turn the section stands in the plane through the axis, its `v` along z.
unit mm
use std
use std.place (vertical)
use std.shapes
use std.stations

component Section(c: point, size: Length, wall: Length) {
  private outer := std.shapes.CenteredRectangle(c, w: size, h: size)
  private inner := std.shapes.CenteredRectangle(c, w: size - 2 * wall, h: size - 2 * wall)
  profile := face(outer.loop, holes: inner.loop)
}

// the section at station `s`: square to the arc there, its `v` along the bend's axis
component Elbow(g: arc, up: axis, s: Scalar) {
  st := std.stations.OnArc(g, up: up, s: s)
  sec := Section(st.n.origin, size: 18mm, wall: 2mm) in st.n
}

in std.top {
  o := point
  fix((0, 0)) o
  entry := point
  fix((30, 0)) entry
  exit := point hint((0, 30))
  o vertical exit
  construction centerline guide := arc(center: o, start: entry, end: exit)
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Elbow(guide, std.z).sec.profile at s })
