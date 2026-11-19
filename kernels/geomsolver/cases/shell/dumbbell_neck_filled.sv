// A dumbbell plate: two 10 × 10 squares joined by a neck 4 long and 2 wide, its outline drawn in
// the top plane and extruded 6, shelled to walls 1.5 thick and open through its top, `part.near`.
// The neck is narrower than twice the wall, so it is left solid, and each square's pocket bulges
// toward the neck on arcs of radius 1.5 about the neck's mouth corners: the shell keeps every
// point within 1.5 of a face kept.
unit mm
use std
use std.solids
use std.rounds (shell)

in std.top {
  p0 := point
  fix((0, 0)) p0
  p1 := point hint((10, 0))
  p2 := point hint((10, 4))
  p3 := point hint((14, 4))
  p4 := point hint((14, 0))
  p5 := point hint((24, 0))
  p6 := point hint((24, 10))
  p7 := point hint((14, 10))
  p8 := point hint((14, 6))
  p9 := point hint((10, 6))
  p10 := point hint((10, 10))
  p11 := point hint((0, 10))
  outline := horizontal line(p0, p1) -> vertical line(p1, p2) -> horizontal line(p2, p3) ->
             vertical line(p3, p4) -> horizontal line(p4, p5) -> vertical line(p5, p6) ->
             horizontal line(p6, p7) -> vertical line(p7, p8) -> horizontal line(p8, p9) ->
             vertical line(p9, p10) -> horizontal line(p10, p11) -> vertical line(p11, p0) -> close
  // each square 10 × 10, the neck 4 long and 2 wide, centred on the squares
  p0 distance(10) p1
  p0 distance(10) p11
  p11 distance(10) p10
  p2 distance(4) p3
  p2 distance(2) p9
  p1 distance(4) p2
  p4 distance(10) p5
  p5 distance(10) p6
  p3 distance(4) p4
  p9 distance(4) p8
}

part := solid(std.solids.Prism(outline, from: 0mm, to: 6mm))
part.near shell(t: 1.5mm) part
