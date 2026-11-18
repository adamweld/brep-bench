// A cube of side 10 from the origin, every edge chamfered 2 on each face, each corner cut flat by
// a triangle. The footprint is drawn in the top plane (x, y) and the prism over it runs to z = 10;
// `chamfer(d: 2mm)` cuts every edge among its faces, each flat its own, so the three at a corner
// meet in a point (chamfer/cube-all-edges-mitered). The library has no flat closing a corner, so a
// solid of the chamfered cube is cut at each corner by the tetrahedron between the cube's corner
// and the triangle through the three inset squares' corners nearest it — on each face through the
// corner, the point 2 in from both of its edges there: at (10, 10, 10) the triangle (8, 8, 10),
// (8, 10, 8), (10, 8, 8), in the plane x + y + z = 26.
unit mm
use std
use std.solids
use std.solids (cut)
use std.sets
use std.rounds (chamfer)

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 10))
  d := point hint((0, 10))
  footprint := horizontal line(a, b) -> vertical line(b, c) ->
               horizontal line(c, d) -> vertical line(d, a) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(10) c
}

cube := solid(std.solids.Prism(footprint, from: 0mm, to: 10mm))
chamfer(d: 2mm) cube

part := solid(cube)
// each corner by which way it lies from the cube's middle along x, y and z
corners := [{x: -1, y: -1, z: -1}, {x: 1, y: -1, z: -1}, {x: 1, y: 1, z: -1}, {x: -1, y: 1, z: -1},
            {x: -1, y: -1, z: 1}, {x: 1, y: -1, z: 1}, {x: 1, y: 1, z: 1}, {x: -1, y: 1, z: 1}]
repeat k in corners {
  tip := point
  on_x := point
  on_y := point
  on_z := point
  fix((5mm + 5mm * k.x, 5mm + 5mm * k.y, 5mm + 5mm * k.z)) tip
  fix((5mm + 5mm * k.x, 5mm + 3mm * k.y, 5mm + 3mm * k.z)) on_x
  fix((5mm + 3mm * k.x, 5mm + 5mm * k.y, 5mm + 3mm * k.z)) on_y
  fix((5mm + 3mm * k.x, 5mm + 3mm * k.y, 5mm + 5mm * k.z)) on_z
  corner := solid({ p | some q in std.sets.Segment(tip, on_x), r in std.sets.Segment(on_y, on_z):
    p in std.sets.Segment(q, r) })
  corner cut part
}
