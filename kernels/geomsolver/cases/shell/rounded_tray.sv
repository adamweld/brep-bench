// The box from the origin to (20, 14, 10) with its four upright edges rounded to radius 2: a plate
// whose outline is the 20 × 14 rectangle with its corners rounded to radius 2, 10 high. Shelled to
// walls 1 thick and open through its top, each corner's arc is moved in about its own centre to a
// radius of 1, the inner walls going on tangent as the outer ones do.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.top {
  // the 20 × 14 rectangle's corners, and its outline: each side a line, each corner an arc of
  // radius 2 going on tangent into the sides beside it
  a := point
  c := point hint((20, 14))
  fix((0, 0)) a
  fix((20, 14)) c
  p0 := point hint((2, 0))
  p1 := point hint((18, 0))
  p2 := point hint((20, 2))
  p3 := point hint((20, 12))
  p4 := point hint((18, 14))
  p5 := point hint((2, 14))
  p6 := point hint((0, 12))
  p7 := point hint((0, 2))
  c1 := point hint((18, 2))
  c2 := point hint((18, 12))
  c3 := point hint((2, 12))
  c4 := point hint((2, 2))
  horizontal (south := line(p0, p1))
  (se := arc(center: c1, from: p1, to: p2))
  vertical (east := line(p2, p3))
  (ne := arc(center: c2, from: p3, to: p4))
  horizontal (north := line(p4, p5))
  (nw := arc(center: c3, from: p5, to: p6))
  vertical (west := line(p6, p7))
  (sw := arc(center: c4, from: p7, to: p0))
  radius(2mm) se
  radius(2mm) ne
  radius(2mm) nw
  radius(2mm) sw
  south tangent se
  east tangent se
  east tangent ne
  north tangent ne
  north tangent nw
  west tangent nw
  west tangent sw
  south tangent sw
  a level(up) p0
  a level(right) p7
  c level(up) p4
  c level(right) p3
}

part := solid(std.solids.Prism(face(south, se, east, ne, north, nw, west, sw), from: 0mm, to: 10mm))
part.near shell(t: 1mm) part
