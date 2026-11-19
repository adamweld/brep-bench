// A curved block slotted by a pin swung through it. The block is the ring 8 to 12 from the z axis,
// |z| <= 1, its section in the front plane turned 60 degrees about z from +x toward +y. The pin is
// a cylinder of radius 0.5 along the x axis from x = 6 to x = 11, drawn as its half section turned
// about its own axis; it turns right-handed about z from -140 to +140 degrees, and everything it
// passes through, the kernel's sweep of the pin under that turn, is cut from the block.
unit mm
use std
use std.solids (cut)
use std.place (coords)
use std.motion (turns)

// the pin's half section, in the plane through its axis (x) and z
private lift := axis
fix(dir == (0, 0, 1)) lift
private out := axis
fix(dir == (1, 0, 0)) out
holder := plane(u: out, v: lift)
fix(origin == (0, 0, 0)) holder
in holder {
  private e0 := point
  private e1 := point hint((6, 0.5))
  private e2 := point hint((11, 0.5))
  private e3 := point hint((11, 0))
  e0 coords(du: 6mm, dv: 0mm) holder
  e1 coords(du: 6mm, dv: 0.5mm) holder
  e0 distance(5mm) e3
  horizontal (spindle := line(e3, e0)) -> vertical (base := line(e0, e1)) ->
  horizontal (side := line(e1, e2)) -> vertical (tip := line(e2, e3)) -> close
}
construction pin := solid(std.solids.Revolution(face(base, side, tip, spindle), about: spindle))

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

// the turn about the block's axis, and the slot the pin sweeps
swing := frame
swing turns(1) ax
construction slot := solid({ p | some t in (-140deg, 140deg): p in pin at(under: swing) t })
slot cut part
