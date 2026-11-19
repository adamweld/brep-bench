// A block, the box from the origin to (20, 10, 10), its section drawn in the front plane (x, z)
// and its prism taken from y = 0 to y = 10 (the front's normal is −y); the edge where its top
// meets its end x = 20 blended by a parabola, a conic of rho 0.5. `std.rounds` has no conic blend
// (and a bound on measurements in a hand-written one must be linear in the distances, which a
// conic's is not), so the blend is made as the case describes it: its section, the quadratic
// Bézier from the top's tangent point (17, 10) through the edge's point (20, 10) to the end's
// (20, 7), traced from its formula; the corner outside that curve a face, swept along the edge
// past both ends and cut from the block.
unit mm
use std
use std.solids
use std.solids (cut)
use std.place (offset)

// the quadratic Bézier over its three control points, at `t`
component Bezier2(p0: point, p1: point, p2: point, t: Scalar) {
  p := point(x: (1 - t)^2 * p0.x + 2 * t * (1 - t) * p1.x + t^2 * p2.x,
             y: (1 - t)^2 * p0.y + 2 * t * (1 - t) * p1.y + t^2 * p2.y)
}

in std.front {
  section := step(u: 20mm, v: 0mm) (bottom := line) -> step(v: 10mm) (end := line) ->
             step(u: 0mm) (top := line) -> step(v: 0mm) (back := line) -> close
  corner := point hint((20, 10))
  corner offset(dx: 20mm, dy: 10mm) std.origin
  on_top := point hint((17, 10))
  on_top offset(dx: -3mm, dy: 0mm) corner
  on_end := point hint((20, 7))
  on_end offset(dx: 0mm, dy: -3mm) corner
  from_top := point hint((17, 10))
  from_top coincident(t == 0) parabola
  to_end := point hint((20, 7))
  to_end coincident(t == 1) parabola
  // the cutter's section: the corner outside the curve, reaching 1 past the block's top and end
  // so that it shares no face with them
  out_end := point hint((21, 7))
  out_end offset(dx: 1mm, dy: 0mm) on_end
  out_corner := point hint((21, 11))
  out_corner offset(dx: 1mm, dy: 1mm) corner
  out_top := point hint((17, 11))
  out_top offset(dx: 0mm, dy: 1mm) on_top
  rim := line(to_end, out_end) -> line(out_end, out_corner) -> line(out_corner, out_top) ->
         line(out_top, from_top)
  outside := face(parabola from from_top to to_end, rim)
}
parabola := trace(Bezier2(on_top, corner, on_end).p over t in (0, 1))

blend := solid(std.solids.Prism(outside, from: -11mm, to: 1mm))
part := solid(std.solids.Prism(section, from: -10mm, to: 0mm))
blend cut part
