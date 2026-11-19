// A block, the box from (-6, -6, -8) to (36, 26, 0), grooved by a ring carried along a spline.
// The path is the cubic Bezier in the block's top through the control points (0, 0), (10, 0),
// (20, 20), (30, 20); `std.stations.Along` is the frame riding it, x along its tangent and z up,
// the world's own frame at its start. The ring is drawn in that starting frame: a torus about the
// line through (0, 0, 0.5) along y, its tube (radius 0.5) 1.5 from that line, standing across
// the path like a wheel. The kernel's sweep of the ring as the frame runs the whole path (the
// spline's own parameter, 0 to 1) is cut from the block.
unit mm
use std
use std.solids (cut)
use std.place (coords, on_u)
use std.shapes
use std.stations

in std.top {
  private k0 := point
  private k1 := point
  private k2 := point
  private k3 := point
  fix((0, 0)) k0
  fix((10, 0)) k1
  fix((20, 20)) k2
  fix((30, 20)) k3
  route := spline(k0, k1, k2, k3)
  private mid := point
  fix((15, 10)) mid
  private outline := std.shapes.CenteredRectangle(mid, w: 42mm, h: 32mm)
}
part := solid(std.solids.Prism(outline.loop, from: -8mm, to: 0mm))
along := std.stations.Along(route, up: std.z)

// the ring, drawn in the level plane through its centre (0, 0, 0.5), its axis along y
private east := axis
fix(dir == (1, 0, 0)) east
private north := axis
fix(dir == (0, 1, 0)) north
bench := plane(u: east, v: north)
fix(origin == (0, 0, 0.5)) bench
in bench {
  private a0 := point
  private a1 := point
  a0 coords(du: 0mm, dv: -1mm) bench
  a1 coords(du: 0mm, dv: 1mm) bench
  private ring_axis := line(a0, a1)
  private tc := point
  tc on_u(d: 1.5mm) bench
  private tube := circle(center: tc) hint(r: 0.5)
  radius(0.5mm) tube
}
construction ring := solid(std.solids.Revolution(face(tube), about: ring_axis))

construction groove := solid({ p | some t in (0rad, 1rad): p in ring at(under: along.f) t })
groove cut part
