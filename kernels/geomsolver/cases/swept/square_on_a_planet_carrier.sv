// The solid a square bar fills as it is carried round on a planet. The carrier turns about z by
// t; the planet, riding it, turns about the carrier's pin (5 out along x, carried round with it)
// by -3t against it, so the bar turns -2t in the world. The bar is the square of side 2 about the
// pin, sides along x and y, drawn in the planet's block, and the prism over it from z = 0 to 2;
// it rides both frames, so the part is the kernel's sweep of the bar as the roll t runs 0..360
// degrees.
unit mm
use std
use std.solids
use std.place (coords)
use std.shapes
use std.motion (spins)

in std.top {
  private o := point
  fix((0, 0)) o
}
in (carrier := frame) {
  in std.top {
    private pin := point
    pin coords(du: 5mm, dv: 0mm) std.top
  }
  in (planet := frame) {
    in std.top {
      private c := point
      c coords(du: 5mm, dv: 0mm) std.top
      private sq := std.shapes.CenteredRectangle(c, w: 2mm, h: 2mm)
    }
    construction bar := solid(std.solids.Prism(sq.loop, from: 0mm, to: 2mm))
  }
}
carrier spins(1) o
planet spins(-3) pin
part := solid({ p | some t in (0deg, 360deg): p in bar at t })
