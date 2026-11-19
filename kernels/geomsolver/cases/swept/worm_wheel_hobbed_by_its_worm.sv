// A worm wheel's 30 teeth hobbed by its worm. The blank is the disc of radius 32 about z from
// z = -6 to 6. The worm's axis is the line through (0, 40, 0) along x. Its thread is the
// trapezoid of a module-2 rack tooth - pi wide at radius 10, from radius 7.5 to 12.5, its sides
// 20 degrees from square to the axis - drawn in the half-plane through the axis toward +y and
// carried up a right-hand screw of lead 2 pi about the axis (the kernel's sweep of a face up a
// screw), kept between the planes x = -20 and 20. The worm turns t about its axis while the
// wheel turns t/30 about z; the kernel's sweep of the thread as the wheel sees it, over thirty
// turns of the worm, is cut from the blank.
unit mm
use std
use std.solids (cut, common)
use std.place (coords)
use std.shapes
use std.motion (turns, screws)

in std.top {
  private o := point
  fix((0, 0)) o
  private rim := circle(center: o) hint(r: 32)
  radius(32mm) rim
}
part := solid(std.solids.Prism(face(rim), from: -6mm, to: 6mm))

// the worm's axis, and the half-plane through it toward +y the thread's section is drawn in
private along := axis
fix(dir == (1, 0, 0)) along
private out := axis
fix(dir == (0, 1, 0)) out
section_plane := plane(u: along, v: out)
fix(origin == (0, 40, 0)) section_plane
in section_plane {
  private w0 := point
  private w1 := point
  w0 coords(du: -1mm, dv: 0mm) section_plane
  w1 coords(du: 1mm, dv: 0mm) section_plane
  construction centerline worm_axis := line(w0, w1)
  // the tooth: pi / 2 either side at radius 10, narrowing 2.5 tan 20 each way out to 12.5 and
  // widening as much in to 7.5
  private r0 := point
  private r1 := point
  private r2 := point
  private r3 := point
  r0 coords(du: -(pi / 2 + 2.5 * tan(20deg)) * 1mm, dv: 7.5mm) section_plane
  r1 coords(du: (pi / 2 + 2.5 * tan(20deg)) * 1mm, dv: 7.5mm) section_plane
  r2 coords(du: (pi / 2 - 2.5 * tan(20deg)) * 1mm, dv: 12.5mm) section_plane
  r3 coords(du: -(pi / 2 - 2.5 * tan(20deg)) * 1mm, dv: 12.5mm) section_plane
  private root := line(r0, r1)
  private lead_flank := line(r1, r2)
  private tip := line(r2, r3)
  private trail_flank := line(r3, r0)
  tooth := face(root, lead_flank, tip, trail_flank)
}
// the thread: the tooth carried up the screw past both ends, kept between x = -20 and 20
lead := frame
lead screws(2 * pi * 1mm) worm_axis
construction thread := solid({ p | some t in (-1440deg, 1440deg): p in tooth at(under: lead) t })
in std.side {
  private m := point
  fix((40, 0)) m
  private span := std.shapes.CenteredRectangle(m, w: 40mm, h: 40mm)
}
construction ends := solid(std.solids.Prism(span.loop, from: -20mm, to: 20mm))
ends common thread

// the worm turning about its axis, the wheel a thirtieth as fast about z, and what the thread
// passes through as the wheel sees it over thirty turns of the worm
worm := frame
worm turns(1) worm_axis
wheel := frame
wheel turns(1 / 30) std.z
construction hob := solid({ p | some t in (0deg, 10800deg): p in thread at(under: worm, seen_from: wheel) t })
hob cut part
