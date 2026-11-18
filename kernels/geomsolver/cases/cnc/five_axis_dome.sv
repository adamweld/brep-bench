// A dome machined in five axes by four cutters, as the program moves them relative to the part:
// the blank less everything the cutters pass through, each cutting move its own sweep — the cutter
// (a solid of revolution, drawn once about its own tip and seated where the move takes it up, its
// axis along the move's) carried by the move's own motion: a slide, a turn about the z axis, or a
// turn about a level line through the origin (the trunnion) — cut from the blank. The moves the
// program makes clear of the blank (at 90 or 70 from the origin, or at z = 50) pass through
// nothing of it and are left out.
//
// n(λ, φ) = (cos λ cos φ, cos λ sin φ, sin λ), the dome's outward normal at latitude λ, longitude φ.
// 1. Roughing, a bull-nose mill of radius 8 (corner 2), upright: at each level z = 41, 36, … 6,
//    each circle R = ρ + 9 + 12m < 58 (ρ the dome's radius there), largest first: plunged at
//    (R, 0) from z = 50, turned once round the z axis, lifted.
// 2. Finishing, a ball-end mill of radius 4 square to the dome: along each meridian φ = 0°, 2°, …
//    358°, slid in along n(3°, φ) from 90 to 40, turned 72° about the level line along
//    (sin φ, −cos φ, 0), slid out along n(75°, φ) to 90; round the pole at λ = 75° … 89°, slid in
//    along n(λ, 0°), turned once round the z axis, slid out; and at the pole, in and out along z.
// 3. Spot faces, a flat end mill of radius 5, along n(40°, φ), φ = 15°, 45°, … 345°: in from 70 to
//    38.5 and out. 4. Holes, a drill of radius 2.5 with a 90° point, the same: in from 70 to 30 and out.
unit mm
use std
use std.solids
use std.solids (cut)
use std.place (coords, seats)
use std.motion (slides)

// the blank, the solid cylinder of radius 50 about the z axis from z = 0 to 45
in std.top {
  c := point
  fix((0, 0)) c
  rim := circle(center: c) hint(r: 50)
  radius(50mm) rim
}
part := solid(std.solids.Prism(face(rim), from: 0mm, to: 45mm))

o := point
fix((0, 0, 0)) o
zenith := point
fix((0, 0, 1)) zenith
z_line := line(o, zenith)                  // the table's axis

// the cutters, each drawn about its own z axis, its tip at its origin
component BullNose(r: Length, corner: Length, len: Length) {
  in std.front {
    e1 := point
    k := point
    e2 := point hint((8, 2))
    e3 := point
    e4 := point
    e1 coords(du: r - corner, dv: 0mm) std.front
    k coords(du: r - corner, dv: corner) std.front
    k level(v) e2
    e3 coords(du: r, dv: len) std.front
    e4 coords(du: 0mm, dv: len) std.front
    bottom := line(std.origin, e1)
    round := arc(center: k, start: e1, end: e2)
    side := line(e2, e3)
    top := line(e3, e4)
    spindle := line(e4, std.origin)
  }
  body := solid(std.solids.Revolution(face(bottom, round, side, top, spindle), about: spindle))
}
component BallEnd(r: Length, len: Length) {
  in std.front {
    k := point
    e1 := point hint((4, 4))
    e2 := point
    e3 := point
    k coords(du: 0mm, dv: r) std.front
    k level(v) e1
    e2 coords(du: r, dv: len) std.front
    e3 coords(du: 0mm, dv: len) std.front
    ball := arc(center: k, start: std.origin, end: e1)
    side := line(e1, e2)
    top := line(e2, e3)
    spindle := line(e3, std.origin)
  }
  body := solid(std.solids.Revolution(face(ball, side, top, spindle), about: spindle))
}
component FlatEnd(r: Length, len: Length) {
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

// a cutter posed with its tip `d` out along n(lat, lon) and its axis along that normal, for the
// tool `kind` (2 the ball-end mill, 3 the flat end mill, 4 the drill)
component Posed(kind: Int, lat: Angle, lon: Angle, d: Length) {
  tip := point
  fix((d * cos(lat) * cos(lon), d * cos(lat) * sin(lon), d * sin(lat))) tip
  normal := axis
  fix(dir == (cos(lat) * cos(lon), cos(lat) * sin(lon), sin(lat))) normal
  across := axis
  fix(dir == (-sin(lon), cos(lon), 0)) across
  when kind == 2 {
    cutter := BallEnd(r: 4mm, len: 40mm)
    cutter seats(turn: 0deg, up: normal, east: across) tip
    body := solid(cutter.body)
  }
  when kind == 3 {
    cutter := FlatEnd(r: 5mm, len: 40mm)
    cutter seats(turn: 0deg, up: normal, east: across) tip
    body := solid(cutter.body)
  }
  when kind == 4 {
    cutter := Drill(r: 2.5mm, len: 40mm)
    cutter seats(turn: 0deg, up: normal, east: across) tip
    body := solid(cutter.body)
  }
}

// a slide along the cutter's own axis, its tip from `d0` to `d1` out along n(lat, lon)
component Plunge(blank: solid, kind: Int, lat: Angle, lon: Angle, d0: Length, d1: Length) {
  at := Posed(kind: kind, lat: lat, lon: lon, d: d0)
  way := axis
  fix(dir == (sign * cos(lat) * cos(lon), sign * cos(lat) * sin(lon), sign * sin(lat))) way
  sign := if(d1 > d0, 1, -1)
  at.tip coincident way
  feed := frame
  feed slides(360mm) way
  swept := solid({ p | some t in (0deg, abs(d1 - d0) / 1mm * 1deg): p in at.body at(under: feed) t })
  swept cut blank
}

// a turn about `pivot` by `sweep`, the cutter's tip `d` out along n(lat, lon)
component Turn(blank: solid, kind: Int, lat: Angle, lon: Angle, d: Length, pivot: line,
               sweep: Angle) {
  at := Posed(kind: kind, lat: lat, lon: lon, d: d)
  swept := solid({ p | some t in (0deg, sweep): p in at.body at(under: pivot) t })
  swept cut blank
}

// 1. roughing: the bull-nose upright (latitude 90° is along z), at (R, 0, z)
component Ring(blank: solid, z_axis: line, R: Length, z: Length) {
  tip := point
  fix((R, 0mm, z)) tip
  up := axis
  fix(dir == (0, 0, 1)) up
  east := axis
  fix(dir == (1, 0, 0)) east
  mill := BullNose(r: 8mm, corner: 2mm, len: 40mm)
  mill seats(turn: 0deg, up: up, east: east) tip
  down := axis
  fix(dir == (0, 0, -1)) down
  tip coincident down
  feed := frame
  feed slides(360mm) down
  plunge := solid({ p | some t in (0deg, (50mm - z) / 1mm * 1deg): p in mill.body at(under: feed) t })
  plunge cut blank            // the plunge from z = 50 and, the same column, the lift back to it
  lap := solid({ p | some t in (0deg, 360deg): p in mill.body at(under: z_axis) t })
  lap cut blank
}
repeat zi in [41mm, 36mm, 31mm, 26mm, 21mm, 16mm, 11mm, 6mm] {
  rho := sqrt(max(0mm^2, 1600mm^2 - zi^2))
  repeat 5 as m {
    R := rho + 9mm + 12mm * (4 - m)                     // largest first
    when R < 58mm {
      Ring(part, z_line, R: R, z: zi)
    }
  }
}

// 2. finishing along the meridians, round the pole and at it
repeat 180 as k {
  phi := 2deg * k
  q := point
  fix((sin(phi) * 1mm, -cos(phi) * 1mm, 0mm)) q
  trunnion := line(o, q)
  Plunge(part, kind: 2, lat: 3deg, lon: phi, d0: 90mm, d1: 40mm)
  Turn(part, kind: 2, lat: 3deg, lon: phi, d: 40mm, pivot: trunnion, sweep: 72deg)
  Plunge(part, kind: 2, lat: 75deg, lon: phi, d0: 40mm, d1: 90mm)
}
repeat 15 as k {
  lam := 75deg + 1deg * k
  Plunge(part, kind: 2, lat: lam, lon: 0deg, d0: 90mm, d1: 40mm)
  Turn(part, kind: 2, lat: lam, lon: 0deg, d: 40mm, pivot: z_line, sweep: 360deg)
  Plunge(part, kind: 2, lat: lam, lon: 0deg, d0: 40mm, d1: 90mm)
}
Plunge(part, kind: 2, lat: 90deg, lon: 0deg, d0: 90mm, d1: 40mm)
Plunge(part, kind: 2, lat: 90deg, lon: 0deg, d0: 40mm, d1: 90mm)

// 3. spot faces and 4. holes, at latitude 40°
repeat 12 as k {
  phi := 15deg + 30deg * k
  Plunge(part, kind: 3, lat: 40deg, lon: phi, d0: 70mm, d1: 38.5mm)
  Plunge(part, kind: 3, lat: 40deg, lon: phi, d0: 38.5mm, d1: 70mm)
}
repeat 12 as k {
  phi := 15deg + 30deg * k
  Plunge(part, kind: 4, lat: 40deg, lon: phi, d0: 70mm, d1: 30mm)
  Plunge(part, kind: 4, lat: 40deg, lon: phi, d0: 30mm, d1: 70mm)
}
