+++
title = "A smooth loft through a triangle, a square, an octagon, a square and a triangle, between two points"
tags = ["loft", "smooth", "point-ends", "polygons", "unequal-sections", "unsettled"]
origin = "OCCT thrusection/specific/F2"
settled = false
+++

A smooth loft through five stacked polygons of different numbers of sides, capped at each end by a point: a tower that swells from a triangle to an octagon and back, its two ends closing to tips.

The sections are regular polygons centered on the z axis, in horizontal planes:

| z | polygon | circumradius | a corner at azimuth |
|---|---|---|---|
| 0 | triangle | 2.5 | 90° |
| 2 | square | 3.5 | 45° |
| 4 | octagon | 4 | 0° |
| 6 | square | 3.5 | 45° |
| 8 | triangle | 2.5 | 270° |

The end points are $P_0 = (0.6, 0.4, -2)$, 2 below the first polygon, and $P_6 = (-0.4, -0.6, 10)$, 2 above the last. Together with the polygons they make seven sections, $k = 0 \dots 6$, at the heights z = −2, 0, 2, …, 10.

Each polygon is taken as a closed polyline of 24 stations, $S_j$ for $j = 0 \dots 23$, the station $S_j$ being the point of its boundary at azimuth 15°·j about the z axis (a ray from the axis at that azimuth meets the boundary once); every corner of every polygon is one of its stations. For each station j there is a curve $Q_j(t)$, $t \in [0, 1]$, through the seven points $S_j^k$, k = 0 … 6, taken at $t = k/6$ ($S_j^0 = P_0$ and $S_j^6 = P_6$ for every j, and $S_j^k$ the station j of the polygon k for k = 1 … 5): the natural cubic spline, each of its three coordinates a piecewise cubic with two continuous derivatives that has zero second derivative at t = 0 and at t = 1. Its height is z = −2 + 12t exactly. The loft is the surface of the points $(1 - f)\,Q_j(t) + f\,Q_{j+1}(t)$ for j = 0 … 23 (j + 1 taken round), f in [0, 1], t in [0, 1]: at each t the polyline through the points $Q_j(t)$, filled straight between stations. At t = 0 and t = 1 the polyline has shrunk to the point $P_0$ and the point $P_6$, so the solid is closed by this surface alone: it has no flat faces.
