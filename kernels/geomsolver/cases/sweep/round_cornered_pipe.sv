// A round pipe turned round a sharp corner with a round joint. The path is drawn in the top
// plane: from (-30, 0) along +x to the origin, then a right angle to the left and on along +y to
// (0, 30). Each run is the disk of radius 3 drawn square to it, slid along it from end to end; at
// the corner the disk square to the first run is turned a quarter turn about the upright through
// the corner, the z axis, until it stands square to the second, and the three are united: the
// half of it outside the corner fills the wedge the runs leave open there, a quarter ball.
unit mm
use std
use std.place (horizontal, on_v)
use std.solids
use std.solids (union)
use std.stations

in std.top {
  a := point
  fix((-30, 0)) a
  o := point
  fix((0, 0)) o
  c := point hint((0, 30))
  horizontal (first := line(a, o))
  second := line(o, c)
  length(30mm) second
  first angle(90deg) second
}

// the disk square to the path at a station of it, its `v` along z
component Disk(g: line, at: Scalar) {
  st := std.stations.OnLine(g, up: std.z, s: at)
  in st.n {
    rim := circle(center: st.n.origin)
    radius(3mm) rim
  }
  f := face(rim)
}

start := Disk(first, at: 0)
turned := Disk(second, at: 0)

// at the corner, the half of the disk outside the corner: the station's `u` runs to the first
// run's left, toward the second, so it is the half on the side of -u, its arc from `top` round
// anticlockwise to `bottom`; the other half turns inside the runs, and a turn about a line across
// a face is refused
at_corner := std.stations.OnLine(first, up: std.z, s: 1)
in at_corner.n {
  top := point
  bottom := point
  top on_v(d: 3mm) at_corner.n
  bottom on_v(d: -3mm) at_corner.n
  outer := arc(center: at_corner.n.origin, start: top, end: bottom)
  diameter := line(bottom, top)
}

// each run its disk slid along its normal (which faces along the run), 30 from end to end
run1 := solid(std.solids.Prism(start.f, from: 0mm, to: 30mm))
run2 := solid(std.solids.Prism(turned.f, from: 0mm, to: 30mm))
joint := solid(std.solids.Revolution(face(outer, diameter), about: std.z, to: 90deg))

part := solid(run1)
run2 union part
joint union part
