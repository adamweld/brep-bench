// A pocketed tray with four standoffs, in three steps as the case says: the 72 × 48 block 16 deep
// (its face drawn in the front plane, the prism behind it along +y), the 66 × 42 pocket 13 deep cut
// from it and named as the hollowed shell, and four tubes standing on the pocket's floor added to
// that shell, each a ring's prism 8 tall so its hole is blind.
unit mm
use std
use std.solids (cut, union)

width := 72mm
height := 48mm
depth := 16mm
wall := 3mm
boss_radius := 5mm
screw_radius := 2mm
boss_height := 8mm

component Rectangle(center: point, w: Length, h: Length) {
  a := point hint((center.x - w / 2, center.y - h / 2))
  b := point hint((center.x + w / 2, center.y - h / 2))
  c := point hint((center.x + w / 2, center.y + h / 2))
  d := point hint((center.x - w / 2, center.y + h / 2))
  profile := horizontal (ab := line(a, b)) -> vertical (bc := line(b, c)) ->
            horizontal (cd := line(c, d)) -> vertical (da := line(d, a)) -> close
  length(w) ab
  length(h) bc
  diagonal := line(a, c)
  center midpoint diagonal
}

in std.front {
  outer := Rectangle(std.origin, w: width, h: height)
  inner := Rectangle(std.origin, w: width - 2 * wall, h: height - 2 * wall)
  block := solid(std.solids.Prism(outer.profile, from: -depth, to: 0))
  pocket := solid(std.solids.Prism(inner.profile, from: -(depth - wall), to: 0))
  pocket cut block
  part := solid(block)
  // the standoffs' axes on an inset rectangle: each clears the pocket's walls by 2
  centers := Rectangle(std.origin, w: width - 2 * (wall + boss_radius + 2mm),
                                h: height - 2 * (wall + boss_radius + 2mm))
}

component Standoff(body: solid, c: point, r: Length, screw_r: Length, floor: Length, h: Length) {
  rim := circle(center: c)
  screw := circle(center: c)
  radius(r) rim
  radius(screw_r) screw
  boss := solid(std.solids.Prism(face(rim, holes: screw), from: floor, to: floor + h))
  boss union body
}
in std.front {
  repeat c in [centers.a, centers.b, centers.c, centers.d] {
    Standoff(part, c, r: boss_radius, screw_r: screw_radius, floor: -depth + wall, h: boss_height)
  }
}
