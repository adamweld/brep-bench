// The lens common to two NURBS balls of radius 5 about (-3, 0, 0) and (3, 0, 0). Each ball's
// surface is the rational quadratic meridian of `geometry/nurbs-sphere` — the semicircle from its
// south pole to its north on five control points weighing 1, √2/2, 1, √2/2, 1, with a double knot
// at its middle — carried round the upright through its centre; Solvent has no surface patch, so
// each ball is its meridian, drawn in the front plane (x, z) on the +x side of its centre (its
// seam), turned a whole turn about that upright. A spline here is a cubic, so each quadratic piece
// is raised to one exactly: its inner control points the fraction 2 - √2 of the way from each end
// toward the middle point, each weighing (1 + √2) / 3.
unit mm
use std
use std.place (coords)
use std.solids
use std.solids (common)

heavy := (1 + sqrt(2)) / 3

// the ball about `o`: its meridian on the net o + (0, -5), (5, -5), (5, 0), (5, 5), (0, 5), turned
// about the upright through `o`
component NurbsBall(o: point) {
  in o.plane {
    p0 := point
    p1 := point
    p2 := point
    p3 := point
    p4 := point
    o distance(0mm, along: u) p0
    o distance(-5mm, along: v) p0
    o distance(5mm, along: u) p1
    o distance(-5mm, along: v) p1
    o distance(5mm, along: u) p2
    o distance(0mm, along: v) p2
    o distance(5mm, along: u) p3
    o distance(5mm, along: v) p3
    o distance(0mm, along: u) p4
    o distance(5mm, along: v) p4
    construction leg0 := line(p0, p1)
    construction leg1 := line(p2, p1)
    construction leg2 := line(p2, p3)
    construction leg3 := line(p4, p3)
    a := point
    b := point
    c := point
    d := point
    a coincident(t == 2 - sqrt(2)) leg0
    b coincident(t == 2 - sqrt(2)) leg1
    c coincident(t == 2 - sqrt(2)) leg2
    d coincident(t == 2 - sqrt(2)) leg3
    meridian := spline(p0, a, b, p2, c, d, p4) knots [0, 0, 0, 0, 0.5, 0.5, 0.5, 1, 1, 1, 1] weights [1, heavy, heavy, 1, heavy, heavy, 1]
    construction centerline spindle := line(p4, p0)
  }
  body := solid(std.solids.Revolution(face(meridian, spindle), about: spindle))
}

in std.front {
  west := point
  fix((-3, 0)) west
  east := point
  fix((3, 0)) east
}
left := NurbsBall(west)
right := NurbsBall(east)

part := solid(left.body)
right.body common part
