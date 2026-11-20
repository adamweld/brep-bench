+++
title = "A frustum lofted between two rounded squares of NURBS, shrunk by 0.5"
tags = ["offset", "shrink", "loft", "nurbs", "ruled"]
origin = "OCCT tests/bugs/modalg_2/bug5805_45, OCCT tests/bugs/modalg_2/bug5805_46"
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

Shrink the whole solid by 0.5, no face opened: the solid of every point at least 0.5 from its
boundary (its four trapezoids, four cone patches, top and bottom).

The corner cones are oblique, whose parallel surface has no closed form, so no volume or area is
stated.

**As a set:**

$$S \ominus B_{1/2}$$
