+++
title = "A frustum lofted between two rounded squares of NURBS, shelled with its narrow top open"
tags = ["shell", "inward", "loft", "nurbs", "ruled"]
origin = "OCCT tests/bugs/modalg_2/bug5805_46"
settled = false
+++

A solid made by a ruled loft between a rounded square and a copy of it scaled up and moved, closed
by the two flat faces they bound. The upper curve lies in the plane z = 8: the square with
corners (±3, ±3, 8), each corner rounded by a quarter circle of radius 1 about (±2, ±2, 8), as one
closed rational B-spline curve (the quarter circles are exact in it). The lower curve is that
curve scaled by 2 about the point (0, 0, 8), then moved down to the plane z = 0 and shifted by
(1, 0.5): a rounded square of half-side 6 and corner radius 2 centered at (1, 0.5, 0). The loft
joins each point of the upper curve to its image on the lower one by a straight line, so each
straight side becomes a planar trapezoid and each corner a slanted cone.

Shell it inward to a wall 0.5 thick, open at its narrow top face, z = 8. The side's moved faces
run on past the plane of the opening and are cut by it: the side is taken as continued beyond
z = 8, and the rim is where its offset meets the plane z = 8. Exactly: let E be the solid
continued up to z = 10 by the same lines, the rounded square at height z being the lower one
scaled by (2 − z/8)/2 about its center, which runs from (1, 0.5) at z = 0 to (0, 0) at z = 8 and
on to (−0.25, −0.125) at z = 10, closed by the rounded square at z = 10. The cavity is E shrunk by
0.5, every point of E at least 0.5 from E's boundary (its side, its floor and its far end), cut off
at the plane z = 8; the far end is more than 0.5 above it. The result is the solid less that cavity.

The inward offset of a slanted cone is not a cone, so the cavity's wall has no closed form and no
volume or area is stated.

**As a set:**

$$S \setminus \bigl((E \ominus B_{1/2}) \cap \{z \le 8\}\bigr), \quad E \text{ the solid continued to } z = 10,\ S = E \cap \{z \le 8\}$$
