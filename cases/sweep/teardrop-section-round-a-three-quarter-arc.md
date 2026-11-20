+++
title = "A teardrop section swept along a three-quarter circle"
tags = ["sweep", "planar-path", "circular-path", "bezier-section", "unsettled"]
origin = "OCCT pipe/standard/F1"
settled = false
+++

A pipe whose section is a closed cubic Bezier curve, swept along a circular arc.

The path is the arc of the circle of radius 15 about the z axis in the xy plane, from (15, 0, 0)
through (0, 15, 0) and (−15, 0, 0) to (0, −15, 0): three quarters of a turn, leaving the start
along +y.

The section, at every point of the path, is the closed cubic Bezier curve of four control points
Q0, Q1, Q2, Q3 = Q0, a teardrop with a corner at its foot, in the plane square to the path and
placed in coordinates (r, z) about the path point, r measured outward from the z axis: Q0 = Q3 =
(0, −2), Q1 = (8, 3), Q2 = (−8, 3). The curve leaves its foot Q0 to the right and rises, rounds
over the top at z = 1.75 and comes down to its foot from the left; it is widest, 2.31 either side
of the path point, a little above the foot, and it has a corner at the foot. It encloses an area
of 12 and does not cross itself. At the start, its control points are (15, 0, −2), (23, 0, 3),
(7, 0, 3) and (15, 0, −2). The two ends of the pipe are flat, each the section there.
