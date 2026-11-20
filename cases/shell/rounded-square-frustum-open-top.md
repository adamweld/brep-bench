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

Shell it inward to a wall 0.5 thick, open at its narrow top face, z = 8. The cavity is every point
of the solid at least 0.5 from its side and at least 0.5 above its bottom, running up to the top
plane, which is the opening; the wall is everything else in the solid. The result is the solid less
that cavity.

The inward offset of a slanted cone is not a cone, so the cavity's wall has no closed form and no
volume or area is stated.

**As a set:**

$$S \setminus \{\, p \in S : d(p,\ \partial S \cap \{z < 8\}) \ge \tfrac12 \,\}, \quad S \text{ the lofted solid}$$
