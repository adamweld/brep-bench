// A 2.5-axis program of three tools, as the CNC program cuts it: the plate less everything the
// cutters pass through, each cutting move its own sweep — the tool (a solid of revolution, drawn
// once about its own tip and seated where the move takes it up, facing the way it goes) carried by
// the move's own motion, a slide (G1) along its line or a turn (G2, G3) about the upright line
// through its arc's centre — cut from the plate. The moves between cuts, the tip at z = 5 above
// the plate, pass through nothing of it and are left out.
//
// The pocket, at each depth z = −4, −8, −12 with a flat end mill of radius 5: raster passes along
// y = 26 + 4k, k = 0 … 14, each over the interval(s) where the line crosses the region D inside the
// wall path (the rectangle (25, 25)–(135, 85), its corners rounded to 7) and outside the island path
// (the circle of radius 20 about (60, 55)) — each plunged at its left end, run to its right and
// lifted; then once round the wall path anticlockwise from (80, 25), then once round the island
// path clockwise from (80, 55). The bore, a flat end mill of radius 3: at z = −3 and then −6, out
// from (60, 55) to (66, 55), a whole circle anticlockwise and back. The holes, a drill of radius 4
// with a 90° point, pecking at the plate's four corners.
unit mm
use std
use std.solids
use std.solids (cut)
use std.place (coords, seats)
use std.motion (slides)

// the plate, the box from (0, 0, −20) to (160, 110, 0)
in std.top {
  b0 := point
  b1 := point
  b2 := point
  b3 := point
  b0 coords(du: 0mm, dv: 0mm) std.top
  b1 coords(du: 160mm, dv: 0mm) std.top
  b2 coords(du: 160mm, dv: 110mm) std.top
  b3 coords(du: 0mm, dv: 110mm) std.top
  plate_f := face(b0, b1, b2, b3, -> close)
}
part := solid(std.solids.Prism(plate_f, from: -20mm, to: 0mm))

// a flat end mill of radius `r`, `len` long from its tip, drawn about its own z axis, its tip at
// its origin
component Mill(r: Length, len: Length) {
  in std.front {
    e1 := point
    e2 := point
    e3 := point
    e1 coords(du: r, dv: 0mm) std.front
    e2 coords(du: r, dv: len) std.front
    e3 coords(du: 0mm, dv: len) std.front
    bottom := line(std.origin, e1)
    side := line(e1, e2)
    top := line(e2, e3)
    spindle := line(e3, std.origin)
  }
  body := solid(std.solids.Revolution(face(bottom, side, top, spindle), about: spindle))
}

// a drill of radius `r`, `len` long from its tip: a cone of half-angle 45° from its tip up to its
// side, `r` above it
component Drill(r: Length, len: Length) {
  in std.front {
    e1 := point
    e2 := point
    e3 := point
    e1 coords(du: r, dv: r) std.front
    e2 coords(du: r, dv: len) std.front
    e3 coords(du: 0mm, dv: len) std.front
    point_ := line(std.origin, e1)
    side := line(e1, e2)
    top := line(e2, e3)
    spindle := line(e3, std.origin)
  }
  body := solid(std.solids.Revolution(face(point_, side, top, spindle), about: spindle))
}

// a straight move (G1) of `tool`, its tip at `x0`, slid to `x1`
component Straight(tool: group, x0: group, x1: group) {
  len := hypot(hypot(x1.x - x0.x, x1.y - x0.y), x1.z - x0.z)
  tip := point
  fix((x0.x, x0.y, x0.z)) tip
  mill := Mill(r: tool.r, len: tool.len)
  mill seats(turn: atan2(x1.y - x0.y, x1.x - x0.x), up: std.z, east: std.x) tip   // facing its way
  way := axis
  fix(dir == ((x1.x - x0.x) / len, (x1.y - x0.y) / len, (x1.z - x0.z) / len)) way
  tip coincident way
  feed := frame
  feed slides(360mm) way
  swept := solid({ p | some t in (0deg, len / 1mm * 1deg): p in mill.body at(under: feed) t })
}

// an arc move (G2 clockwise, G3 anticlockwise, seen from above) of `tool`, its tip at `x0`, turned
// by `sweep` about the upright line through the arc's centre `c`
component Arc(tool: group, x0: group, c: group, sweep: Angle, ccw: Int) {
  heading := atan2(x0.y - c.y, x0.x - c.x) + if(ccw == 1, 90deg, -90deg)   // along the arc
  tip := point
  fix((x0.x, x0.y, x0.z)) tip
  mill := Mill(r: tool.r, len: tool.len)
  mill seats(turn: heading, up: std.z, east: std.x) tip
  centre := point
  fix((c.x, c.y, x0.z)) centre
  above := point
  fix((c.x, c.y, x0.z + if(ccw == 1, 1mm, -1mm))) above
  pivot := line(centre, above)              // turning right-handed about it: anticlockwise if up
  swept := solid({ p | some t in (0deg, sweep): p in mill.body at(under: pivot) t })
}

// a drill's peck: slid along its axis from `z0` to `z1` over (x, y)
component Peck(tool: group, x: Length, y: Length, z0: Length, z1: Length) {
  tip := point
  fix((x, y, z0)) tip
  drill := Drill(r: tool.r, len: tool.len)
  drill seats(turn: 0deg, up: std.z, east: std.x) tip
  way := axis
  fix(dir == (0, 0, if(z1 > z0, 1, -1))) way
  tip coincident way
  feed := frame
  feed slides(360mm) way
  swept := solid({ p | some t in (0deg, abs(z1 - z0) / 1mm * 1deg): p in drill.body at(under: feed) t })
}

pocket_mill := {r: 5mm, len: 30mm}
bore_mill := {r: 3mm, len: 30mm}
drill := {r: 4mm, len: 30mm}
clear := 5mm                               // the tip's height between cuts

// the pocket at one depth
component Level(body: solid, tool: group, z: Length) {
  // the raster passes
  repeat 15 as k {
    y := 26mm + 4mm * k
    rise := max(0mm, max(32mm - y, y - 78mm))                  // into a corner's rounding
    wall_l := if(rise > 0mm, 32mm - sqrt(49mm^2 - rise^2), 25mm)
    wall_r := if(rise > 0mm, 128mm + sqrt(49mm^2 - rise^2), 135mm)
    when abs(y - 55mm) < 20mm {                                 // across the island path
      hx := sqrt(400mm^2 - (y - 55mm)^2)
      a_in := Straight(tool, {x: wall_l, y: y, z: clear}, {x: wall_l, y: y, z: z})
      a_run := Straight(tool, {x: wall_l, y: y, z: z}, {x: 60mm - hx, y: y, z: z})
      a_out := Straight(tool, {x: 60mm - hx, y: y, z: z}, {x: 60mm - hx, y: y, z: clear})
      b_in := Straight(tool, {x: 60mm + hx, y: y, z: clear}, {x: 60mm + hx, y: y, z: z})
      b_run := Straight(tool, {x: 60mm + hx, y: y, z: z}, {x: wall_r, y: y, z: z})
      b_out := Straight(tool, {x: wall_r, y: y, z: z}, {x: wall_r, y: y, z: clear})
      a_in.swept cut body
      a_run.swept cut body
      a_out.swept cut body
      b_in.swept cut body
      b_run.swept cut body
      b_out.swept cut body
    }
    when abs(y - 55mm) >= 20mm {
      c_in := Straight(tool, {x: wall_l, y: y, z: clear}, {x: wall_l, y: y, z: z})
      c_run := Straight(tool, {x: wall_l, y: y, z: z}, {x: wall_r, y: y, z: z})
      c_out := Straight(tool, {x: wall_r, y: y, z: z}, {x: wall_r, y: y, z: clear})
      c_in.swept cut body
      c_run.swept cut body
      c_out.swept cut body
    }
  }
  // the wall path, anticlockwise from (80, 25): each side to its corner's rounding and round it
  w_in := Straight(tool, {x: 80mm, y: 25mm, z: clear}, {x: 80mm, y: 25mm, z: z})
  w_in.swept cut body
  corners := [{x0: 80mm, y0: 25mm, x1: 128mm, y1: 25mm, cx: 128mm, cy: 32mm},
              {x0: 135mm, y0: 32mm, x1: 135mm, y1: 78mm, cx: 128mm, cy: 78mm},
              {x0: 128mm, y0: 85mm, x1: 32mm, y1: 85mm, cx: 32mm, cy: 78mm},
              {x0: 25mm, y0: 78mm, x1: 25mm, y1: 32mm, cx: 32mm, cy: 32mm}]
  repeat w in corners {
    side := Straight(tool, {x: w.x0, y: w.y0, z: z}, {x: w.x1, y: w.y1, z: z})
    side.swept cut body
    round := Arc(tool, {x: w.x1, y: w.y1, z: z}, {x: w.cx, y: w.cy}, sweep: 90deg, ccw: 1)
    round.swept cut body
  }
  w_close := Straight(tool, {x: 32mm, y: 25mm, z: z}, {x: 80mm, y: 25mm, z: z})
  w_close.swept cut body
  w_out := Straight(tool, {x: 80mm, y: 25mm, z: z}, {x: 80mm, y: 25mm, z: clear})
  w_out.swept cut body
  // the island path, a whole circle clockwise from (80, 55)
  i_in := Straight(tool, {x: 80mm, y: 55mm, z: clear}, {x: 80mm, y: 55mm, z: z})
  i_in.swept cut body
  i_lap := Arc(tool, {x: 80mm, y: 55mm, z: z}, {x: 60mm, y: 55mm}, sweep: 360deg, ccw: 0)
  i_lap.swept cut body
  i_out := Straight(tool, {x: 80mm, y: 55mm, z: z}, {x: 80mm, y: 55mm, z: clear})
  i_out.swept cut body
}
repeat z in [-4mm, -8mm, -12mm] {
  Level(part, pocket_mill, z: z)
}

// the bore: plunged at (60, 55), out 6 and a whole circle at each of two depths, and lifted
bore_in := Straight(bore_mill, {x: 60mm, y: 55mm, z: clear}, {x: 60mm, y: 55mm, z: -3mm})
bore_in.swept cut part
repeat z in [-3mm, -6mm] {
  out := Straight(bore_mill, {x: 60mm, y: 55mm, z: z}, {x: 66mm, y: 55mm, z: z})
  out.swept cut part
  ring := Arc(bore_mill, {x: 66mm, y: 55mm, z: z}, {x: 60mm, y: 55mm}, sweep: 360deg, ccw: 1)
  ring.swept cut part
  back := Straight(bore_mill, {x: 66mm, y: 55mm, z: z}, {x: 60mm, y: 55mm, z: z})
  back.swept cut part
}
bore_down := Straight(bore_mill, {x: 60mm, y: 55mm, z: -3mm}, {x: 60mm, y: 55mm, z: -6mm})
bore_down.swept cut part
bore_out := Straight(bore_mill, {x: 60mm, y: 55mm, z: -6mm}, {x: 60mm, y: 55mm, z: clear})
bore_out.swept cut part

// the holes, pecked at each corner
pecks := [{z0: 5mm, z1: -5mm}, {z0: -5mm, z1: 2mm}, {z0: 2mm, z1: -10mm}, {z0: -10mm, z1: 2mm},
          {z0: 2mm, z1: -14mm}, {z0: -14mm, z1: 5mm}]
repeat h in [{x: 8mm, y: 8mm}, {x: 152mm, y: 8mm}, {x: 152mm, y: 102mm}, {x: 8mm, y: 102mm}] {
  repeat k in pecks {
    peck := Peck(drill, x: h.x, y: h.y, z0: k.z0, z1: k.z1)
    peck.swept cut part
  }
}
