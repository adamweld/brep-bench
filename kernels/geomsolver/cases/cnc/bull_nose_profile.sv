// A groove profiled round a six-lobed flower by a bull-nose mill in three step-downs, as the CNC
// program cuts it: the plate less everything the mill passes through, each move of the program its
// own sweep — the mill (a solid of revolution, drawn once about its own tip and seated where the
// move takes it up) carried by the move's own motion, a slide along its line (the plunges and the
// rise) or a turn about the upright line through its arc's centre (each lobe G3, each valley G2) —
// cut from the plate. The flower's arcs are worked out from its centres and radii; lobe 0 is run in
// two pieces, from its tip and back to it.
//
// geomsolver v0.1.24 refuses it: the turn closing the first lap is "refused, not returned wrong" (the
// built solid holds a point the sweep's definition leaves out). A single bull-nose turn about an upright
// line builds or not by where its interval stands round the line (0° to 90° from +x builds, −90°
// to 0° does not, whichever way the mill is seated), and a slide square to its axis by which way
// the mill is seated about its own axis, so the mill is seated facing the way it goes.
unit mm
use std
use std.solids
use std.solids (cut)
use std.place (coords, seats)
use std.motion (slides)

// the plate, the box from (0, 0, −16) to (120, 120, 0)
in std.top {
  b0 := point
  b1 := point
  b2 := point
  b3 := point
  b0 coords(du: 0mm, dv: 0mm) std.top
  b1 coords(du: 120mm, dv: 0mm) std.top
  b2 coords(du: 120mm, dv: 120mm) std.top
  b3 coords(du: 0mm, dv: 120mm) std.top
  plate_f := face(b0, b1, b2, b3, -> close)
}
part := solid(std.solids.Prism(plate_f, from: -16mm, to: 0mm))

// a bull-nose mill of radius 4, its end corner rounded to 1.5, 25 long from its tip: its half
// section — out along its flat end, round the corner, up its side, across its top, down its axis —
// drawn about its own z axis, its tip at its origin, and turned
component Mill() {
  in std.front {
    e1 := point
    k := point
    e2 := point hint((4, 1.5))
    e3 := point
    e4 := point
    e1 coords(du: 2.5mm, dv: 0mm) std.front
    k coords(du: 2.5mm, dv: 1.5mm) std.front
    k level(v) e2                       // the corner a quarter round, from under its centre
    e3 coords(du: 4mm, dv: 25mm) std.front
    e4 coords(du: 0mm, dv: 25mm) std.front
    bottom := line(std.origin, e1)
    corner := arc(center: k, start: e1, end: e2)
    side := line(e2, e3)
    top := line(e3, e4)
    spindle := line(e4, std.origin)
  }
  body := solid(std.solids.Revolution(face(bottom, corner, side, top, spindle), about: spindle))
}

// a straight move (G1): the mill, its tip at `x0`, slid to `x1`
component Straight(x0: group, x1: group) {
  len := hypot(hypot(x1.x - x0.x, x1.y - x0.y), x1.z - x0.z)
  tip := point
  fix((x0.x, x0.y, x0.z)) tip
  mill := Mill()
  mill seats(turn: atan2(x1.y - x0.y, x1.x - x0.x), up: std.z, east: std.x) tip   // facing its way
  way := axis
  fix(dir == ((x1.x - x0.x) / len, (x1.y - x0.y) / len, (x1.z - x0.z) / len)) way
  tip coincident way
  feed := frame
  feed slides(360mm) way
  swept := solid({ p | some t in (0deg, len / 1mm * 1deg): p in mill.body at(under: feed) t })
}
// an arc move (G2 clockwise, G3 anticlockwise, seen from above): the mill, its tip at `x0`, turned
// about the upright line through the arc's centre `c` to `x1`
component Arc(x0: group, x1: group, c: group, ccw: Int) {
  r2 := (x0.x - c.x)^2 + (x0.y - c.y)^2
  sweep := acos(((x0.x - c.x) * (x1.x - c.x) + (x0.y - c.y) * (x1.y - c.y)) / r2)
  heading := atan2(x0.y - c.y, x0.x - c.x) + if(ccw == 1, 90deg, -90deg)   // along the arc
  tip := point
  fix((x0.x, x0.y, x0.z)) tip
  mill := Mill()
  mill seats(turn: heading, up: std.z, east: std.x) tip        // facing its way
  centre := point
  fix((c.x, c.y, x0.z)) centre
  above := point
  fix((c.x, c.y, x0.z + if(ccw == 1, 1mm, -1mm))) above
  pivot := line(centre, above)              // turning right-handed about it: anticlockwise if up
  swept := solid({ p | some t in (0deg, sweep): p in mill.body at(under: pivot) t })
}

// the flower about (60, 60): lobe i an arc of radius 12 about the point 30 out at 60i degrees,
// valley i an arc of radius d − 12 about the point 40 out at 60i + 30 degrees, each tangent to the
// lobes beside it where the line between their centres meets them
d := sqrt(2500 - 1200 * sqrt(3)) * 1mm
component Lap(body: solid, z: Length) {
  repeat 6 as i {
    a := 60deg * i
    lx := 60mm + 30mm * cos(a)                       // lobe i's centre
    ly := 60mm + 30mm * sin(a)
    vx := 60mm + 40mm * cos(a + 30deg)               // valley i's centre, after lobe i
    vy := 60mm + 40mm * sin(a + 30deg)
    px := 60mm + 40mm * cos(a - 30deg)               // valley i − 1's, before it
    py := 60mm + 40mm * sin(a - 30deg)
    nx := 60mm + 30mm * cos(a + 60deg)               // lobe i + 1's
    ny := 60mm + 30mm * sin(a + 60deg)
    tin := {x: lx + 12mm * (px - lx) / d, y: ly + 12mm * (py - ly) / d, z: z}     // into lobe i
    tout := {x: lx + 12mm * (vx - lx) / d, y: ly + 12mm * (vy - ly) / d, z: z}    // out of it
    tnext := {x: nx + 12mm * (vx - nx) / d, y: ny + 12mm * (vy - ny) / d, z: z}   // into lobe i + 1
    when i == 0 {                                    // the lap starts and ends at lobe 0's tip
      tip := {x: lx + 12mm, y: ly, z: z}
      first := Arc(tip, tout, {x: lx, y: ly}, ccw: 1)
      first.swept cut body
      last := Arc(tin, tip, {x: lx, y: ly}, ccw: 1)
      last.swept cut body
    }
    when i > 0 {
      lobe := Arc(tin, tout, {x: lx, y: ly}, ccw: 1)
      lobe.swept cut body
    }
    valley := Arc(tout, tnext, {x: vx, y: vy}, ccw: 0)
    valley.swept cut body
  }
}

// plunge at lobe 0's tip, a lap at each depth, plunging between them, and rise
plunges := [{z0: 5mm, z1: -4mm}, {z0: -4mm, z1: -8mm}, {z0: -8mm, z1: -10mm}, {z0: -10mm, z1: 5mm}]
repeat h in plunges {
  pl := Straight({x: 102mm, y: 60mm, z: h.z0}, {x: 102mm, y: 60mm, z: h.z1})
  pl.swept cut part
}
repeat z in [-4mm, -8mm, -10mm] {
  Lap(part, z: z)
}
