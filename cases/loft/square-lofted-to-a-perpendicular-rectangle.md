+++
title = "A square lofted to a rectangle in a perpendicular plane"
tags = ["loft", "ruled", "skew", "perpendicular-sections"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestLoft.py::testSimpleAdditiveLoftCase"
settled = false
+++

A ruled loft from a square lying flat to a rectangle standing upright, in a plane at a right angle to it.

The square is in the plane z = 0, with corners P0 = (0, 0, 0), P1 = (10, 0, 0), P2 = (10, 10, 0) and
P3 = (0, 10, 0). The rectangle is in the plane x = 30, with corners Q0 = (30, 2, 30), Q1 = (30, 2, 10),
Q2 = (30, 8, 10) and Q3 = (30, 8, 30).

The loft is **ruled**: each side of the square is joined to the matching side of the rectangle (P0P1 to
Q0Q1, P1P2 to Q1Q2, P2P3 to Q2Q3, P3P0 to Q3Q0), the point a fraction s of the way along the square's side
to the point the same fraction s along the rectangle's, by a straight line. The solid is bounded by the
square, the rectangle, and these four ruled surfaces. The sides P1P2 and P3P0 are parallel to Q1Q2 and Q3Q0
(all along y), so those two surfaces are flat trapezoids; the other two, P0P1 to Q0Q1 and P2P3 to Q2Q3,
join lines that are skew to each other, so each is a twisted (hyperbolic paraboloid) patch.
