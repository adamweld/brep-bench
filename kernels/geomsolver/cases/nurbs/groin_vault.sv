// A groin vault: the common of two barrel-vaulted blocks on the square |x|, |y| <= 12, the second
// the first turned a quarter turn about the z axis. Each block's top is the rational B-spline of
// degree 1 along its run and 2 across it: so each block is its section, drawn across its run and
// slid along it — the floor, two upright sides 10 high and the rational quadratic Bézier arc on
// (-12, 10), (0, 26) and (12, 10) weighing 1, 3/5 and 1, exactly the arc of radius 15 about a point
// 1 above the floor. A spline here is a cubic, so the quadratic is raised to one exactly: its
// inner control points the fraction 2w / (1 + 2w) of the way from each end toward the middle one,
// each weighing (1 + 2w) / 3, w = 3/5.
unit mm
use std
use std.solids
use std.solids (common)

w := 3 / 5
heavy := (1 + 2 * w) / 3

// the first block's section across y, in the side plane (y, z), slid along x
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
along_x := solid(std.solids.Prism(face(floor, east, arch, west), from: -12mm, to: 12mm))

// the second: the same section across x, in the front plane (x, z), slid along y
in std.front {
  f := point
  g := point
  h := point
  top := point
  j := point
  fix((-12, 0)) f
  fix((12, 0)) g
  fix((12, 10)) h
  fix((0, 26)) top
  fix((-12, 10)) j
  construction leg2 := line(h, top)
  construction leg3 := line(j, top)
  i := point
  i2 := point
  i coincident(t == 2 * w / (1 + 2 * w)) leg2
  i2 coincident(t == 2 * w / (1 + 2 * w)) leg3
  sole := line(f, g)
  north := line(g, h)
  crossing := spline(h, i, i2, j) weights [1, heavy, heavy, 1]
  south := line(j, f)
}
along_y := solid(std.solids.Prism(face(sole, north, crossing, south), from: -12mm, to: 12mm))

part := solid(along_x)
along_y common part
