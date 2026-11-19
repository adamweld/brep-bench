// A spiroid face gear's 30 teeth hobbed by its tapered worm. The blank is the given quadrilateral
// in the front plane (x, z) turned whole about z: a ring from radius 40 to 52 with a conical face.
// The worm's axis is the line through (0, 30, 6) along x, its pitch cone R(x) = 6 + x tan 5.
// Its thread is a face family: at each station s (radians), in the half-plane through the axis
// turned s about it, the rack tooth centred at x = s - pi wide at the pitch cone, from 2.5 inside
// it to 2.5 outside, its sides 20 degrees from square to the axis - whose corners ride the cone as
// it grows; the thread is that section as s runs, kept between x = 0 and 52. The worm turns t
// about its axis while the gear turns t/30 about z; the kernel's sweep of the thread as the gear
// sees it, over thirty turns of the worm, is cut from the blank.
unit mm
use std
use std.solids (cut, common)
use std.place (coords)
use std.shapes
use std.motion (turns)

taper := tan(5deg)
side := tan(20deg)

in std.front {
  private k0 := point
  private k1 := point
  private k2 := point
  private k3 := point
  fix((40, -9)) k0
  fix((52, -9)) k1
  fix((52, -1.7)) k2
  fix((40, -0.3)) k3
  private underside := line(k0, k1)
  private rim := line(k1, k2)
  private face_cone := line(k2, k3)
  private bore := line(k3, k0)
  construction centerline gear_axis := line(std.origin, hint((0, 1)))
  fix((0, 1)) gear_axis.p2
}
part := solid(std.solids.Revolution(face(underside, rim, face_cone, bore), about: gear_axis))

// the worm's axis, and the half-plane through it toward +y the thread's section is drawn in
private along := axis
fix(dir == (1, 0, 0)) along
private out := axis
fix(dir == (0, 1, 0)) out
section_plane := plane(u: along, v: out)
fix(origin == (0, 30, 6)) section_plane
in section_plane {
  private w0 := point
  private w1 := point
  w0 coords(du: -1mm, dv: 0mm) section_plane
  w1 coords(du: 1mm, dv: 0mm) section_plane
  construction centerline worm_axis := line(w0, w1)
}

// the thread: at station s the section stands turned s about the axis, centred s along it, its
// root and tip 2.5 either side of the pitch cone there and its flanks 20 degrees from square
param s: Scalar over (-3, 56)
lay := frame
lay turns(1) worm_axis
fix(value == s * 1rad) lay
in lay {
  in section_plane {
    private r0 := point
    private r1 := point
    private r2 := point
    private r3 := point
    r0 coords(du: (s - pi / 2 - 2.5 * side) * 1mm, dv: (6 + (s - pi / 2 - 2.5 * side) * taper - 2.5) * 1mm) section_plane
    r1 coords(du: (s + pi / 2 + 2.5 * side) * 1mm, dv: (6 + (s + pi / 2 + 2.5 * side) * taper - 2.5) * 1mm) section_plane
    r2 coords(du: (s + pi / 2 - 2.5 * side) * 1mm, dv: (6 + (s + pi / 2 - 2.5 * side) * taper + 2.5) * 1mm) section_plane
    r3 coords(du: (s - pi / 2 + 2.5 * side) * 1mm, dv: (6 + (s - pi / 2 + 2.5 * side) * taper + 2.5) * 1mm) section_plane
    private root := line(r0, r1)
    private lead_flank := line(r1, r2)
    private tip := line(r2, r3)
    private trail_flank := line(r3, r0)
    tooth := face(root, lead_flank, tip, trail_flank)
  }
}
construction thread := solid({ p | some s: p in tooth at s })
in std.side {
  private m := point
  fix((30, 6)) m
  private span := std.shapes.CenteredRectangle(m, w: 40mm, h: 40mm)
}
construction ends := solid(std.solids.Prism(span.loop, from: 0mm, to: 52mm))
ends common thread

// the worm turning about its axis, the gear a thirtieth as fast about z, and what the thread
// passes through as the gear sees it over thirty turns of the worm
worm := frame
worm turns(1) worm_axis
gear := frame
gear turns(1 / 30) gear_axis
construction hob := solid({ p | some t in (0deg, 10800deg): p in thread at(under: worm, seen_from: gear) t })
hob cut part
