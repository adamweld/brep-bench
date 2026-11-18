// A closed hollow dome with a post inside it. The dome: the plate, a disk of radius 20 from
// z = −2 to 0, joined to the hemispherical shell, the points above the top plane between 18 and
// 20 from the origin. The post: the disk of radius 3 on the cavity's floor extruded up until it
// meets the dome's inside — the points within 3 of the z axis, above the floor, within the sphere
// of radius 18 that bounds the cavity — joined to the dome.
unit mm
use std
use std.sets
use std.solids
use std.solids (union)

o := point
fix((0, 0, 0)) o
up := point
fix((0, 0, 1)) up
z_axis := line(o, up)

outer := std.sets.Ball(o, r: 20mm)
cavity := std.sets.Ball(o, r: 18mm)
shell := solid({ p | p in outer; p not in cavity; p distance(>= 0mm, along: n) std.top })

in std.top {
  c := point
  fix((0, 0)) c
  rim := circle(center: c) hint(r: 20)
  radius(20mm) rim
}
part := solid(std.solids.Prism(face(rim), from: -2mm, to: 0mm))
shell union part

post := solid({ p | p in std.sets.Cylinder(z_axis, r: 3mm); p distance(>= 0mm, along: n) std.top; p in cavity })
post union part
