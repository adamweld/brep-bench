+++
title = "A ruled loft through a triangle, a square, an octagon, a square and a triangle, between two points"
tags = ["loft", "ruled", "point-ends", "polygons", "unequal-sections", "unsettled"]
origin = "OCCT thrusection/specific/E5"
settled = false
+++

A ruled loft through five stacked polygons of different numbers of sides, capped by a point at each end, both points displaced sideways: a tower that swells from a triangle to an octagon and back.

The sections are regular polygons centered on the z axis, in horizontal planes:

| z | polygon | circumradius | a corner at azimuth |
|---|---|---|---|
| 0 | triangle | 2.5 | 90° |
| 2 | square | 3.5 | 45° |
| 4 | octagon | 4 | 0° |
| 6 | square | 3.5 | 45° |
| 8 | triangle | 2.5 | 270° |

The end points are (1.5, 1, −3) below the first section and (−1, −1.5, 11) above the last. Where the polygons have different numbers of corners, the straight lines that join them need a rule, and it is this one. Each section is taken as a closed polyline of 24 stations, $S_j$ for $j = 0 \dots 23$, the station $S_j$ being the point of its boundary at azimuth 15°·j about the z axis (a ray from the axis at that azimuth meets the boundary once); every corner of every polygon is one of its stations. Consecutive sections are joined station to station: the strip between the segment $S_j S_{j+1}$ of one section and the segment $S_j S_{j+1}$ of the next is the ruled surface joining the point at each fraction of one segment to the point at the same fraction of the other. Each end point is joined by a straight line to every point of the neighbouring section's boundary. The solid is closed by these surfaces alone.
