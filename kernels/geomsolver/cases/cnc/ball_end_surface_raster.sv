// A free-form surface finished by a ball-end mill in 41 three-axis raster passes, as the CNC
// program cuts it: the plate less everything the mill passes through, each move of the program its
// own sweep — the mill (a solid of revolution, drawn once with its tip at the origin and carried by
// a slide to where the move takes it up) slid along the move's line — cut from the plate.
//
// Pass j runs along y = 2j through the 23 points (−5 + 5i, 2j, f), f = −5 + 4 sin(πx/50) sin(πy/40),
// along +x on even passes and back along −x on odd ones, the tip run straight from each point to
// the next. The plunge at (−5, 0), the moves between passes (at x = −5 or 105) and the rise at
// (−5, 80) keep the mill, of radius 3, clear of the plate (x from 0 to 100) and pass through
// nothing of it, so they are left out.
unit mm
use std
use std.solids
use std.solids (cut)
use std.place (coords)
use std.motion (slides)

// the plate, the box from (0, 0, −20) to (100, 80, 0)
in std.top {
  b0 := point
  b1 := point
  b2 := point
  b3 := point
  b0 coords(du: 0mm, dv: 0mm) std.top
  b1 coords(du: 100mm, dv: 0mm) std.top
  b2 coords(du: 100mm, dv: 80mm) std.top
  b3 coords(du: 0mm, dv: 80mm) std.top
  plate_f := face(b0, b1, b2, b3, -> close)
}
part := solid(std.solids.Prism(plate_f, from: -20mm, to: 0mm))

// a ball-end mill of radius 3, 30 long from its tip: a half ball below a cylinder, flat on top,
// its half section drawn in the front plane about the z axis, its tip at the origin, and turned
in std.front {
  k := point
  e1 := point hint((3, 3))
  e2 := point
  e3 := point
  k coords(du: 0mm, dv: 3mm) std.front
  k level(v) e1                         // the ball a quarter round, from the tip to its equator
  e2 coords(du: 3mm, dv: 30mm) std.front
  e3 coords(du: 0mm, dv: 30mm) std.front
  ball := arc(center: k, start: std.origin, end: e1)
  side := line(e1, e2)
  top := line(e2, e3)
  spindle := line(e3, std.origin)
}
mill := solid(std.solids.Revolution(face(ball, side, top, spindle), about: spindle))
o := point
fix((0, 0, 0)) o

// a straight move (G1): the mill carried from where it is drawn to its tip at `x0`, then slid to `x1`
component Straight(tool: solid, o: point, x0: group, x1: group) {
  d0 := hypot(hypot(x0.x, x0.y), x0.z)
  len := hypot(hypot(x1.x - x0.x, x1.y - x0.y), x1.z - x0.z)
  to_start := axis
  fix(dir == (x0.x / d0, x0.y / d0, x0.z / d0)) to_start
  o coincident to_start
  place := frame
  place slides(360mm) to_start
  at_start := tool at(under: place) (d0 / 1mm * 1deg)
  way := axis
  fix(dir == ((x1.x - x0.x) / len, (x1.y - x0.y) / len, (x1.z - x0.z) / len)) way
  o coincident way
  feed := frame
  feed slides(360mm) way
  swept := solid({ p | some t in (0deg, len / 1mm * 1deg): p in at_start at(under: feed) t })
}

repeat 41 as j {
  y := 2mm * j
  back := j - 2 * floor(j / 2)                          // 0 along +x, 1 back along −x
  repeat 22 as i {
    xa := if(back == 0, -5mm + 5mm * i, 105mm - 5mm * i)
    xb := if(back == 0, xa + 5mm, xa - 5mm)
    za := -5mm + 4mm * sin(180deg * xa / 50mm) * sin(180deg * y / 40mm)
    zb := -5mm + 4mm * sin(180deg * xb / 50mm) * sin(180deg * y / 40mm)
    step := Straight(mill, o, {x: xa, y: y, z: za}, {x: xb, y: y, z: zb})
    step.swept cut part
  }
}
