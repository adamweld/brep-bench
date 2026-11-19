// The inner half of a torus thickened by 1 toward its axis. The sheet is the inner half of the
// torus whose tube, radius 2, runs about the circle of radius 6 about the z axis: the boundary of
// the solid torus, no farther than 6 from the axis. Thickened away from the middle circle it is the
// points within 1 of the solid torus and not in it, within 6 of the axis — its boundary circles
// sweep the cylinder of radius 6 above and below. (Solvent's thickening of a sheet, a developable
// panel's band, takes a ruled panel between two rails; a torus is neither, so the sheet is
// thickened as the offset of the region it bounds.)
unit mm
use std
use std.sets
use std.solids

o := point
fix((0, 0, 0)) o
up := point
fix((0, 0, 1)) up
z_axis := line(o, up)
in std.front {
  m := point
  fix((6, 0)) m
  tube := circle(center: m) hint(r: 2)
  radius(2mm) tube
  disk := face(tube)
}
torus := solid(std.solids.Revolution(disk, about: std.z))
part := solid({ q |
  q distance(<= 1mm) torus
  q not in torus
  q in std.sets.Cylinder(z_axis, r: 6mm)
})
