// A groove profiled round a cross by a flat end mill in three step-downs, as the CNC program cuts
// it: the plate less everything the mill passes through, each move of the program its own sweep —
// the mill (a solid of revolution, drawn once about its own tip and seated where the move takes it
// up) carried by the move's own motion, a slide (G1) along its line or a turn (G2, G3) about the
// upright line through its arc's centre — cut from the plate.
//
// The program: plunge at (60, 10) to z = −3, a lap round the rounded cross, plunge to −6, a lap,
// plunge to −9, a lap, rise to z = 5. A lap is the table `lap`, its moves in order from (60, 10)
// anticlockwise round the outline whose corners are rounded to 6 — each corner's arc (G3 round an
// outside corner, G2 round an inside one) and the straight sides between, the first side in two
// halves — read off the cross's corners and the radius as a CAM program writes them.
//
// The plate less the sweeps is a set, so the order of the cuts says nothing: the laps are listed
// deepest first, the order the kernel builds (cut shallowest first, geomsolver v0.1.24 refuses the
// second lap's 15th move: "the built solid leaves out a point the sweep's definition holds").
unit mm
use std
use std.solids
use std.solids (cut)
use std.place (coords, seats)
use std.motion (slides)

// the plate, the box from (0, 0, −15) to (120, 90, 0)
in std.top {
  b0 := point
  b1 := point
  b2 := point
  b3 := point
  b0 coords(du: 0mm, dv: 0mm) std.top
  b1 coords(du: 120mm, dv: 0mm) std.top
  b2 coords(du: 120mm, dv: 90mm) std.top
  b3 coords(du: 0mm, dv: 90mm) std.top
  plate_f := face(b0, b1, b2, b3, -> close)
}
part := solid(std.solids.Prism(plate_f, from: -15mm, to: 0mm))

// a flat end mill of radius 3, 20 long from its tip, drawn about its own z axis, its tip at its origin
component Mill() {
  in std.front {
    e1 := point
    e2 := point
    e3 := point
    e1 coords(du: 3mm, dv: 0mm) std.front
    e2 coords(du: 3mm, dv: 20mm) std.front
    e3 coords(du: 0mm, dv: 20mm) std.front
    bottom := line(std.origin, e1)
    side := line(e1, e2)
    top := line(e2, e3)
    spindle := line(e3, std.origin)
  }
  body := solid(std.solids.Revolution(face(bottom, side, top, spindle), about: spindle))
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

lap := [
  {g: 1, x0: 60mm, y0: 10mm, x1: 69mm, y1: 10mm, cx: 0mm, cy: 0mm},
  {g: 3, x0: 69mm, y0: 10mm, x1: 75mm, y1: 16mm, cx: 69mm, cy: 16mm},
  {g: 1, x0: 75mm, y0: 16mm, x1: 75mm, y1: 24mm, cx: 0mm, cy: 0mm},
  {g: 2, x0: 75mm, y0: 24mm, x1: 81mm, y1: 30mm, cx: 81mm, cy: 24mm},
  {g: 1, x0: 81mm, y0: 30mm, x1: 94mm, y1: 30mm, cx: 0mm, cy: 0mm},
  {g: 3, x0: 94mm, y0: 30mm, x1: 100mm, y1: 36mm, cx: 94mm, cy: 36mm},
  {g: 1, x0: 100mm, y0: 36mm, x1: 100mm, y1: 54mm, cx: 0mm, cy: 0mm},
  {g: 3, x0: 100mm, y0: 54mm, x1: 94mm, y1: 60mm, cx: 94mm, cy: 54mm},
  {g: 1, x0: 94mm, y0: 60mm, x1: 81mm, y1: 60mm, cx: 0mm, cy: 0mm},
  {g: 2, x0: 81mm, y0: 60mm, x1: 75mm, y1: 66mm, cx: 81mm, cy: 66mm},
  {g: 1, x0: 75mm, y0: 66mm, x1: 75mm, y1: 74mm, cx: 0mm, cy: 0mm},
  {g: 3, x0: 75mm, y0: 74mm, x1: 69mm, y1: 80mm, cx: 69mm, cy: 74mm},
  {g: 1, x0: 69mm, y0: 80mm, x1: 51mm, y1: 80mm, cx: 0mm, cy: 0mm},
  {g: 3, x0: 51mm, y0: 80mm, x1: 45mm, y1: 74mm, cx: 51mm, cy: 74mm},
  {g: 1, x0: 45mm, y0: 74mm, x1: 45mm, y1: 66mm, cx: 0mm, cy: 0mm},
  {g: 2, x0: 45mm, y0: 66mm, x1: 39mm, y1: 60mm, cx: 39mm, cy: 66mm},
  {g: 1, x0: 39mm, y0: 60mm, x1: 26mm, y1: 60mm, cx: 0mm, cy: 0mm},
  {g: 3, x0: 26mm, y0: 60mm, x1: 20mm, y1: 54mm, cx: 26mm, cy: 54mm},
  {g: 1, x0: 20mm, y0: 54mm, x1: 20mm, y1: 36mm, cx: 0mm, cy: 0mm},
  {g: 3, x0: 20mm, y0: 36mm, x1: 26mm, y1: 30mm, cx: 26mm, cy: 36mm},
  {g: 1, x0: 26mm, y0: 30mm, x1: 39mm, y1: 30mm, cx: 0mm, cy: 0mm},
  {g: 2, x0: 39mm, y0: 30mm, x1: 45mm, y1: 24mm, cx: 39mm, cy: 24mm},
  {g: 1, x0: 45mm, y0: 24mm, x1: 45mm, y1: 16mm, cx: 0mm, cy: 0mm},
  {g: 3, x0: 45mm, y0: 16mm, x1: 51mm, y1: 10mm, cx: 51mm, cy: 16mm},
  {g: 1, x0: 51mm, y0: 10mm, x1: 60mm, y1: 10mm, cx: 0mm, cy: 0mm}
]

depths := [-9mm, -6mm, -3mm]
plunges := [{z0: 5mm, z1: -3mm}, {z0: -3mm, z1: -6mm}, {z0: -6mm, z1: -9mm}, {z0: -9mm, z1: 5mm}]
repeat h in plunges {
  pl := Straight({x: 60mm, y: 10mm, z: h.z0}, {x: 60mm, y: 10mm, z: h.z1})
  pl.swept cut part
}
repeat d in depths {
  repeat m in lap {
    when m.g == 1 {
      ln := Straight({x: m.x0, y: m.y0, z: d}, {x: m.x1, y: m.y1, z: d})
      ln.swept cut part
    }
    when m.g > 1 {
      ar := Arc({x: m.x0, y: m.y0, z: d}, {x: m.x1, y: m.y1, z: d}, {x: m.cx, y: m.cy},
                ccw: if(m.g == 3, 1, 0))
      ar.swept cut part
    }
  }
}
