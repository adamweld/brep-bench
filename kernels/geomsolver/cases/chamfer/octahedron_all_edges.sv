// The octahedron |x| + |y| + |z| <= 10, every edge chamfered √6 on each face, and each corner cut
// flat by a square. Its middle square, drawn in the top plane with its corners on the axes 10 from
// the origin, is a prism from z = -10 to z = 10, and from it is cut everything beyond each of its
// eight faces — the plane through three corners, its normal out — by a disc drawn on that plane
// about a corner and swept out along the normal past the prism. Every edge among its faces is then
// chamfered √6, measured across the face square to the edge: the library's `chamfer(d: …)` takes
// its leg as the distance from the other face, so `d` is that leg times the sine of the dihedral
// angle between two faces, cos θ = -1/3. The library's flats mitre at a corner, the four meeting in
// a point (chamfer/octahedron-all-edges-mitered), and it has no flat closing a corner, so a solid
// of the chamfered octahedron is cut by the six corner planes |x|, |y|, |z| <= 6.
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

octahedron := solid(std.solids.Prism(square, from: -10mm, to: 10mm))

ppp := Beyond(px, py, pz)
npp := Beyond(nx, pz, py)
pnp := Beyond(px, pz, ny)
nnp := Beyond(nx, ny, pz)
ppn := Beyond(px, nz, py)
npn := Beyond(nx, py, nz)
pnn := Beyond(px, ny, nz)
nnn := Beyond(nx, nz, ny)
ppp.cutter cut octahedron
npp.cutter cut octahedron
pnp.cutter cut octahedron
nnp.cutter cut octahedron
ppn.cutter cut octahedron
npn.cutter cut octahedron
pnn.cutter cut octahedron
nnn.cutter cut octahedron

chamfer(d: sqrt(6) * sin(acos(-1 / 3)) * 1mm) octahedron

// the chamfered octahedron, each corner then cut flat by the plane 6 from the origin square to
// its axis, |x| <= 6, |y| <= 6, |z| <= 6: a slab beyond it, a disc in the datum plane square to
// that axis swept from 6 to 20 along its normal (the side's +x, the front's -y, the top's +z)
part := solid(octahedron)
in std.side {
  across_x := circle(center: std.side.origin) hint(r: 20)
  radius(20mm) across_x
}
in std.front {
  across_y := circle(center: std.front.origin) hint(r: 20)
  radius(20mm) across_y
}
in std.top {
  across_z := circle(center: std.top.origin) hint(r: 20)
  radius(20mm) across_z
}
east := solid(std.solids.Prism(face(across_x), from: 6mm, to: 20mm))
west := solid(std.solids.Prism(face(across_x), from: -20mm, to: -6mm))
north := solid(std.solids.Prism(face(across_y), from: -20mm, to: -6mm))
south := solid(std.solids.Prism(face(across_y), from: 6mm, to: 20mm))
over := solid(std.solids.Prism(face(across_z), from: 6mm, to: 20mm))
under := solid(std.solids.Prism(face(across_z), from: -20mm, to: -6mm))
east cut part
west cut part
north cut part
south cut part
over cut part
under cut part
