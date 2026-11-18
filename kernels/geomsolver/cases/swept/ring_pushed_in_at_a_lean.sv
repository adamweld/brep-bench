// A block, the box from (-8, -8, -8) to (8, 8, 0), bored by a ring pushed straight down into it at
// a lean. The ring is a torus centred at (0, 0, 4), its axis leaning 35 degrees from upright toward
// +x, its tube (radius 1) 3 from that axis: drawn as the tube's circle in a plane through the
// axis and turned about it. It slides 8 straight down, and the kernel's sweep of it under that
// slide is cut from the block.
unit mm
use std
use std.solids (cut)
use std.place (coords, on_u)
use std.shapes
use std.motion (slides)

lean := 35deg        // how far the ring's axis leans from upright, toward +x

in std.top {
  private mid := point
  fix((0, 0)) mid
  private outline := std.shapes.CenteredRectangle(mid, w: 16mm, h: 16mm)
}
part := solid(std.solids.Prism(outline.loop, from: -8mm, to: 0mm))

// the ring, drawn in the plane through its axis and the x axis, its centre 4 above the top
private along := axis
fix(dir == (sin(lean), 0, cos(lean))) along
private across := axis
fix(dir == (cos(lean), 0, -sin(lean))) across
bench := plane(u: across, v: along)
fix(origin == (0, 0, 4)) bench
in bench {
  private a0 := point
  private a1 := point
  a0 coords(du: 0mm, dv: -1mm) bench
  a1 coords(du: 0mm, dv: 1mm) bench
  private ring_axis := line(a0, a1)
  private tc := point
  tc on_u(d: 3mm) bench
  private tube := circle(center: tc) hint(r: 1)
  radius(1mm) tube
}
construction ring := solid(std.solids.Revolution(face(tube), about: ring_axis))

// the push: 8 straight down, the ring's centre ending at (0, 0, -4)
private down := axis
fix(dir == (0, 0, -1)) down
ram := frame
ram slides(8mm) down
construction bore := solid({ p | some t in (0deg, 360deg): p in ring at(under: ram) t })
bore cut part
