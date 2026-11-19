// A round pipe mitered along three edges of a cube of side 30: the path runs from the origin to
// (30, 0, 0), to (30, 30, 0), to (30, 30, 30). Each run is the disk of radius 2 drawn square to
// it, slid straight along it, flat where the pipe ends and reaching past each corner; at each
// corner the plane through it square to the sum of the two runs' directions, a miter, cuts the
// run coming in from what lies ahead of it and the run going out from what lies behind, and the
// three runs are united.
unit mm
use std
use std.shapes
use std.solids
use std.solids (cut, union)
use std.stations

a := point
b := point
c := point
d := point
fix((0, 0, 0)) a
fix((30, 0, 0)) b
fix((30, 30, 0)) c
fix((30, 30, 30)) d
first := line(a, b)
second := line(b, c)
third := line(c, d)

// a run: the disk square to `g` where its station `at` stands, slid from `from` to `to` along the
// station's normal (which faces along `g`)
component Run(g: line, up: axis, at: Scalar, from: Length, to: Length) {
  st := std.stations.OnLine(g, up: up, s: at)
  in st.n {
    rim := circle(center: st.n.origin)
    radius(2mm) rim
  }
  run := solid(std.solids.Prism(face(rim), from: from, to: to))
}

// the miter at the corner `k` between the runs `into` and `out`: the plane through `k` square to
// the chord between the points 10 either side of it along the path, which runs along the sum of
// the two directions; its normal faces ahead, and a block on either side of it reaches past the
// pipe
component Miter(k: point, into: line, out: line, across: line) {
  p := point
  q := point
  p coincident into
  q coincident out
  p distance(10mm, along: into) k
  k distance(10mm, along: out) q
  chord := line(p, q)
  w := point
  rise := line(k, w)
  length(10mm) rise
  rise parallel(sense: same) across
  e := point
  run_out := line(k, e)
  length(10mm) run_out
  run_out perpendicular chord
  rise perpendicular(sense: cw, about: chord) run_out
  m := plane(u: run_out, v: rise)
  cover := std.shapes.CenteredRectangle(m.origin, w: 40mm, h: 40mm) in m
  ahead := solid(std.solids.Prism(face(cover.loop), from: 0mm, to: 40mm))
  behind := solid(std.solids.Prism(face(cover.loop), from: -40mm, to: 0mm))
}

at_b := Miter(b, first, second, std.z)
at_c := Miter(c, second, third, std.back)

r1 := Run(first, std.z, at: 0, from: 0mm, to: 40mm)
r2 := Run(second, std.z, at: 0, from: -10mm, to: 40mm)
r3 := Run(third, std.x, at: 1, from: -40mm, to: 0mm)

leg1 := solid(r1.run)
at_b.ahead cut leg1
leg2 := solid(r2.run)
at_b.behind cut leg2
at_c.ahead cut leg2
leg3 := solid(r3.run)
at_c.behind cut leg3

part := solid(leg1)
leg2 union part
leg3 union part
