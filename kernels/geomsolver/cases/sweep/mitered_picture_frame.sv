// A picture frame: a rabbeted molding mitered round the 60 × 40 rectangle about the origin of the
// top plane. Each side is the section, drawn square to its run where the run starts (walking the
// rectangle anticlockwise, so its `u` points inward, to the run's left, and its `v` up), slid
// straight along the run past both corners; at each corner the upright plane through the corner
// square to the bisector of the turn, a miter, cuts the side coming in from what lies ahead of it
// and the side going out from what lies behind, and the four sides are united.
unit mm
use std
use std.place (coords)
use std.shapes
use std.solids
use std.solids (cut, union)
use std.stations

in std.top {
  sw := point
  se := point hint((30, -20))
  ne := point hint((30, 20))
  nw := point hint((-30, 20))
  horizontal (south := line(sw, se)) -> vertical (east := line(se, ne)) ->
  horizontal (north := line(ne, nw)) -> vertical (west := line(nw, sw)) -> close
  fix((-30, -20)) sw
  length(60mm) south
  length(40mm) east
}

// the molding run along `g`: in (s, z), s outward from the path (against `u`), the bar 8 wide and
// 6 tall with the rabbet 3 wide and 2 deep taken from its inner bottom edge; slid 100 each way
// along the run
component Side(g: line) {
  st := std.stations.OnLine(g, up: std.z, s: 0)
  in st.n {
    a := point
    b := point
    c := point
    d := point
    e := point
    f := point
    a coords(du: 0mm, dv: 2mm) st.n
    b coords(du: -3mm, dv: 2mm) st.n
    c coords(du: -3mm, dv: 0mm) st.n
    d coords(du: -8mm, dv: 0mm) st.n
    e coords(du: -8mm, dv: 6mm) st.n
    f coords(du: 0mm, dv: 6mm) st.n
  }
  run := solid(std.solids.Prism(face(a, b, c, d, e, f, -> close), from: -100mm, to: 100mm))
}

// the miter at the corner `c` between the runs `into` and `out`: the upright plane through it whose
// normal bisects the turn, and a block reaching past the frame on either side of it
component Miter(c: point, into: line, out: line) {
  in c.plane {
    k := point hint(at: c, along: out, by: 0.1, turn: -45deg)
    bisector := line(c, k)
    length(10mm) bisector
    into angle(bisector, out) bisector
  }
  m := plane(u: hint(dir: bisector, turn: 90deg, about: std.z), v: hint(dir: std.z))
  m.origin coincident c
  m.v parallel std.z
  m.u perpendicular bisector
  m.u perpendicular std.z
  cover := std.shapes.CenteredRectangle(m.origin, w: 200mm, h: 200mm) in m
  ahead := solid(std.solids.Prism(face(cover.loop), from: 0mm, to: 200mm))
  behind := solid(std.solids.Prism(face(cover.loop), from: -200mm, to: 0mm))
}

at_se := Miter(se, south, east)
at_ne := Miter(ne, east, north)
at_nw := Miter(nw, north, west)
at_sw := Miter(sw, west, south)

bottom := Side(south)
right := Side(east)
top := Side(north)
left := Side(west)

s1 := solid(bottom.run)
at_sw.behind cut s1
at_se.ahead cut s1
s2 := solid(right.run)
at_se.behind cut s2
at_ne.ahead cut s2
s3 := solid(top.run)
at_ne.behind cut s3
at_nw.ahead cut s3
s4 := solid(left.run)
at_nw.behind cut s4
at_sw.ahead cut s4

part := solid(s1)
s2 union part
s3 union part
s4 union part
