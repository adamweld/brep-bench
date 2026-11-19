// A hollow square reducer: one hollow square section run along the y axis from y = 0 to y = 40,
// its outside 24 across at the start and 12 at the end, the size running linearly with the
// station, the wall 2 thick all along. At each station the section stands square to the guide,
// its `v` along z, so its sides stay parallel to x and z.
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

// the section at station `s`: square to the guide, its size the inlet's mixed with the outlet's
component Reducer(g: line, up: axis, inlet: Length, outlet: Length, s: Scalar) {
  st := std.stations.OnLine(g, up: up, s: s)
  sec := Section(st.n.origin, size: inlet + (outlet - inlet) * s, wall: 2mm) in st.n
}

in std.top {
  entry := point
  fix((0, 0)) entry
  exit := point hint((0, 40))
  construction centerline guide := vertical line(entry, exit)
  length(40mm) guide
}

param s: Scalar over (0, 1)
part := solid({ p | some s: p in Reducer(guide, std.z, inlet: 24mm, outlet: 12mm).sec.profile at s })
