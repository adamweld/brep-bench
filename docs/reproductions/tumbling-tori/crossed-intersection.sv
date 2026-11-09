// Crossed intersection
// Intersection of opposing sweeps: small lobes and sharp junctions.
// Based on bmander/geomsolver swept_tumble.sv; parameters fixed for reproduction.
unit mm
use std
use std.solids
use std.place (above, coords, on_u, on_v)
use std.motion (turns)

ring := 1mm
tube := 0.22mm
start := -100deg
finish := 100deg
spin_ratio := 0.7
observer_ratio := 1
lean := 45deg
heading := 20deg
torus_x := 3mm
torus_y := 2mm
hub_x := 2mm
observer_height := 0.7071mm

// The torus: a circle turned about an upright axis `ring` away from its centre.
in std.front {
  private ta := point
  private tb := point
  fix((torus_x, torus_y - 1mm)) ta
  fix((torus_x, torus_y + 1mm)) tb
  private taxis := line(ta, tb)
  private tc := point hint((torus_x + ring, torus_y))
  tc distance(ring, along: right) ta
  tc distance(torus_y, along: v) std.front
  private section_c := circle(center: tc) hint(r: tube)
  radius(tube) section_c
  construction torus := solid(std.solids.Revolution(face(section_c), about: taxis))

  // Compound motion about two independently oriented axes.
  private hub := point hint((hub_x, 0))
  hub on_u(d: hub_x) std.front
  private hub_up := point hint((hub_x, 5mm))
  hub_up above(d: 5mm) hub
  construction centerline cradle := line(hub, hub_up)
}
private spin := frame
spin turns(spin_ratio) cradle
private rise := axis hint(dir: (0, cos(lean), sin(lean)))
fix(dir == (0, cos(lean), sin(lean))) rise
private tilted := plane(u: std.x, v: rise)
fix(origin == (0, 0, 0)) tilted
in tilted {
  private k0 := point hint((0, observer_height))
  private k1 := point hint((5mm * cos(heading), observer_height + 5mm * sin(heading)))
  k0 on_v(d: observer_height) tilted
  k1 coords(du: 5mm * cos(heading), dv: observer_height + 5mm * sin(heading)) tilted
  construction centerline kaxis := line(k0, k1)
}
private observer := frame
observer turns(observer_ratio) kaxis
construction tumbled := solid({ p | some t in (start, finish): p in torus at(under: spin, seen_from: observer) t })

private counter := frame
counter turns(-observer_ratio) kaxis
construction other := solid({ p | some t in (start, finish): p in torus at(under: spin, seen_from: counter) t })
body := solid({ p | p in tumbled; p in other })
