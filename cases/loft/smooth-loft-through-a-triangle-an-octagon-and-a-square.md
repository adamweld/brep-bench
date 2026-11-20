+++
title = "A smooth loft through a triangle, an octagon and a square"
tags = ["loft", "smooth", "polygons", "unequal-sections", "unsettled"]
origin = "OCCT thrusection/specific/E7"
settled = false
+++

A smooth loft through three stacked polygons of different numbers of sides, closed by the flat faces of the first and the last: a tower that swells from a triangle to an octagon and narrows to a square.

The sections are regular polygons centered on the z axis, in horizontal planes:

| z | polygon | circumradius | a corner at azimuth |
|---|---|---|---|
| −3 | triangle | 2.5 | 90° |
| 0 | octagon | 4 | 0° |
| 3 | square | 3.5 | 45° |

Each section is taken as a closed polyline of 24 stations, $S_j$ for $j = 0 \dots 23$, the station $S_j$ being the point of its boundary at azimuth 15°·j about the z axis (a ray from the axis at that azimuth meets the boundary once); every corner of every polygon is one of its stations. For each station j there is a quadratic Bézier curve $Q_j(t)$ whose control points are $S_j$ of the triangle, $2S_j$ of the octagon less half the sum of $S_j$ of the triangle and of the square, and $S_j$ of the square: it runs from the triangle's station to the square's and passes through the octagon's at its parameter ½. The loft is the surface of the points $(1 - f)\,Q_j(t) + f\,Q_{j+1}(t)$ for j = 0 … 23 (j + 1 taken round), f in [0, 1], t in [0, 1]: at each t the polyline through the points $Q_j(t)$, filled straight between stations. The solid is closed by the flat triangle at the bottom and the flat square at the top.
