// A hollow square duct narrowing round a bend: one hollow square section carried a quarter turn
// round the arc of radius 30 about the z axis, from (30, 0, 0) to (0, 30, 0), standing at each
// fraction of the turn in the plane through the axis, its `v` along z; its outer half-width runs
// linearly with the angle turned from 9 to 6, and the wall stays 2 thick.
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

// the section at station `s`: square to the arc, its half-width the inlet's mixed with the outlet's
component Bend(g: arc, up: axis, inlet: Length, outlet: Length, s: Scalar) {
  st := std.stations.OnArc(g, up: up, s: s)
  sec := Section(st.n.origin, size: 2 * (inlet + (outlet - inlet) * s), wall: 2mm) in st.n
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
part := solid({ p | some s: p in Bend(guide, std.z, inlet: 9mm, outlet: 6mm).sec.profile at s })
