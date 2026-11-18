// The octahedron |x| + |y| + |z| <= 10: its middle square, drawn in the top plane with its corners
// on the axes 10 from the origin, as a prism from z = -10 to z = 10, and from it cut everything
// beyond each of its eight faces — the plane through three corners, its normal out — by a disc
// drawn on that plane about a corner and swept out along the normal past the prism. Then every
// edge among its faces chamfered √6 on each face, measured across the face square to the edge:
// the library's `chamfer(d: …)` takes its leg as the distance from the other face, so `d` is that
// leg times the sine of the dihedral angle between two faces, cos θ = -1/3. Each flat is its own,
// so the four at a corner run on until they meet in a point, the corner mitred.
unit mm
use std
use std.solids
use std.solids (cut)
use std.rounds (chamfer)

in std.top {
  px := point
  py := point
  nx := point
  ny := point
  fix((10, 0)) px
  fix((0, 10)) py
  fix((-10, 0)) nx
  fix((0, -10)) ny
  square := line(px, py) -> line(py, nx) -> line(nx, ny) -> line(ny, px) -> close
}
pz := point
nz := point
fix((0, 0, 10)) pz
fix((0, 0, -10)) nz

// a face of the octahedron, through corners `a`, `b`, `c` running counter-clockwise seen from
// outside: its plane over its edges from `a`, and the material beyond it, a disc of radius 40
// about `a` swept 40 out along the plane's normal
component Beyond(a: point, b: point, c: point) {
  ab := line(a, b)
  ac := line(a, c)
  flat := plane(u: ab, v: ac)
  in flat {
    rim := circle(center: flat.origin) hint(r: 40)
    radius(40mm) rim
  }
  cutter := solid(std.solids.Prism(face(rim), from: 0mm, to: 40mm))
}

part := solid(std.solids.Prism(square, from: -10mm, to: 10mm))

ppp := Beyond(px, py, pz)
npp := Beyond(nx, pz, py)
pnp := Beyond(px, pz, ny)
nnp := Beyond(nx, ny, pz)
ppn := Beyond(px, nz, py)
npn := Beyond(nx, py, nz)
pnn := Beyond(px, ny, nz)
nnn := Beyond(nx, nz, ny)
ppp.cutter cut part
npp.cutter cut part
pnp.cutter cut part
nnp.cutter cut part
ppn.cutter cut part
npn.cutter cut part
pnn.cutter cut part
nnn.cutter cut part

chamfer(d: sqrt(6) * sin(acos(-1 / 3)) * 1mm) part
