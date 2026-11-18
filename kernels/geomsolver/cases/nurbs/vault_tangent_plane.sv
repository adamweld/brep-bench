// A weighted barrel vault cut by the plane along its crown: the block on the square |x|, |y| <= 12
// whose top is the rational B-spline of degree 1 along x and 2 across it, less everything above the
// plane z = 16, cut by a block standing on that plane and reaching past the vault. The vault is its
// section, drawn across x and slid along it — the floor, two upright sides 10 high and the rational
// quadratic Bézier arc on (-12, 10), (0, 26) and (12, 10) weighing 1, 3/5 and 1, exactly the arc of
// radius 15 about a point 1 above the floor. A spline here is a cubic, so the quadratic is raised
// to one exactly: its inner control points the fraction 2w / (1 + 2w) of the way from each end
// toward the middle one, each weighing (1 + 2w) / 3, w = 3/5.
unit mm
use std
use std.solids
use std.shapes
use std.solids (cut)

w := 3 / 5
heavy := (1 + 2 * w) / 3

// the block's section across x, in the side plane (y, z), slid along x
in std.side {
  a := point
  b := point
  c := point
  apex := point
  e := point
  fix((-12, 0)) a
  fix((12, 0)) b
  fix((12, 10)) c
  fix((0, 26)) apex
  fix((-12, 10)) e
  construction leg0 := line(c, apex)
  construction leg1 := line(e, apex)
  d := point
  d2 := point
  d coincident(t == 2 * w / (1 + 2 * w)) leg0
  d2 coincident(t == 2 * w / (1 + 2 * w)) leg1
  floor := line(a, b)
  east := line(b, c)
  arch := spline(c, d, d2, e) weights [1, heavy, heavy, 1]
  west := line(e, a)
}
part := solid(std.solids.Prism(face(floor, east, arch, west), from: -12mm, to: 12mm))


lid := plane
fix(origin == (0, 0, 16)) lid
fix(dir == (1, 0, 0)) lid.u
fix(dir == (0, 1, 0)) lid.v
cover := std.shapes.CenteredRectangle(lid.origin, w: 40mm, h: 40mm) in lid
above := solid(std.solids.Prism(face(cover.loop), from: 0mm, to: 20mm))
above cut part
