// A drum under a saddle: the round drum of radius 8 about the z axis, standing on the top plane,
// in common with the block under the saddle. The saddle is the bilinear patch on the corners
// (-10, -10, 22.5), (10, -10, -2.5), (-10, 10, -2.5) and (10, 10, 22.5); Solvent has no surface
// patch, so the block is the face family of its sections across x: at u along its two edges
// across x, the straight line between their points there is the patch's line across y, and the
// section is the region of the plane through them between a floor 10 below the top plane (the
// patch dips below z = 0 at two corners), the upright sides y = -10 and y = 10, and that line.
// The drum's floor is the top plane.
unit mm
use std
use std.solids
use std.solids (common)

// the section at `u` along the patch's edges `a` (y = -10) and `b` (y = 10), standing on `base`
component Slice(a: line, b: line, base: plane, u: Scalar) {
  m := point
  m coincident(t == u) a
  n := plane(u: hint(dir: (0, 1, 0)), v: hint(dir: (0, 0, 1)))
  n.u parallel(sense: same) std.y
  n.v parallel(sense: same) std.z
  n.origin coincident m
  in n {
    q0 := point hint((0, 0))
    q1 := point hint((20, 0))
    f0 := point hint((0, -10))
    f1 := point hint((20, -10))
    f0 distance(0mm, along: u) q0
    f1 distance(0mm, along: u) q1
    floor := line(f0, f1)
    east := line(f1, q1)
    top := line(q1, q0)
    west := line(q0, f0)
  }
  f0 coincident base
  f1 coincident base
  q0 coincident a
  q1 coincident b
  f := face(floor, east, top, west)
}

p00 := point
p10 := point
p01 := point
p11 := point
fix((-10, -10, 22.5)) p00
fix((10, -10, -2.5)) p10
fix((-10, 10, -2.5)) p01
fix((10, 10, 22.5)) p11
south := line(p00, p10)
north := line(p01, p11)

deep := plane
fix(origin == (0, 0, -10)) deep
fix(dir == (1, 0, 0)) deep.u
fix(dir == (0, 1, 0)) deep.v

param u: Scalar over (0, 1)
under := solid({ p | some u: p in Slice(south, north, deep).f at u })

// the drum on the top plane, its prism reaching above the saddle
in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 8)
  radius(8mm) rim
}
drum := solid(std.solids.Prism(face(rim), from: 0mm, to: 30mm))
part := solid(drum)
under common part
