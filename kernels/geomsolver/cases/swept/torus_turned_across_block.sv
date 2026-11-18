// A curved block grooved by a leaning torus turned past it. The block is the ring 8 to 12 from
// the z axis, |z| <= 1, its section in the front plane turned 60 degrees about z from +x toward
// +y. The cutter is a torus about the axis through (5, 0, 0) leaning from z toward +y by 25
// degrees, its tube (radius 2) 2.5 from that axis: the tube's circle drawn in the plane through
// the axis and x, and turned about the axis. It turns right-handed about z from -140 to 140
// degrees, and the kernel's sweep of it is cut from the block.
unit mm
use std
use std.solids (cut)
use std.place (coords, on_u)
use std.motion (turns)

lean := 25deg        // how far the cutter's axis leans from upright, toward +y

// the cutter, drawn in the plane through its axis and x
private across := axis
fix(dir == (1, 0, 0)) across
private along := axis
fix(dir == (0, sin(lean), cos(lean))) along
holder := plane(u: across, v: along)
fix(origin == (5, 0, 0)) holder
in holder {
  private ta := point
  private tb := point
  ta coords(du: 0mm, dv: -1mm) holder
  tb coords(du: 0mm, dv: 1mm) holder
  private cutter_axis := line(ta, tb)
  private tc := point
  tc on_u(d: 2.5mm) holder
  private tip := circle(center: tc) hint(r: 2)
  radius(2mm) tip
}
construction cutter := solid(std.solids.Revolution(face(tip), about: cutter_axis))

// the block: the ring's section in the front plane, turned a sixth of a turn about z
in std.front {
  construction centerline ax := line(std.origin, hint((0, 1)))
  fix((0, 1)) ax.p2
  private q0 := point
  private q1 := point hint((12, -1))
  private q2 := point hint((12, 1))
  private q3 := point hint((8, 1))
  q0 coords(du: 8mm, dv: -1mm) std.front
  q0 distance(4mm) q1
  q1 distance(2mm) q2
  horizontal (underside := line(q0, q1)) -> vertical (wall_out := line(q1, q2)) ->
  horizontal (topside := line(q2, q3)) -> vertical (wall_in := line(q3, q0)) -> close
}
part := solid(std.solids.Revolution(face(wall_in, underside, wall_out, topside), about: ax, to: 60deg))

swing := frame
swing turns(1) ax
construction groove := solid({ p | some t in (-140deg, 140deg): p in cutter at(under: swing) t })
groove cut part
