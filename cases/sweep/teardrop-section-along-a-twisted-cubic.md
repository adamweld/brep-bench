+++
title = "A teardrop section swept along a twisted cubic by its Frenet frame"
tags = ["sweep", "space-path", "bezier-section", "bezier-path", "frenet", "torsion", "unsettled"]
origin = "OCCT pipe/standard/F7"
settled = false
+++

A pipe whose section is a closed cubic Bezier curve, swept along a space curve, turned about the
path by the path's Frenet frame.

The path is the cubic Bezier curve of control points (0, 0, 0), (12, 16, 0), (26, 8, 0) and
(24, 0, 16): it leaves the origin along (3, 4, 0), swings around to the right and rises out of
the xy plane to end at (24, 0, 16). Its curvature never vanishes (its radius of curvature is
least, 10.28, a little past the middle) and its torsion never vanishes.

The section, at every point of the path, is the closed cubic Bezier curve of four control points
Q0, Q1, Q2, Q3 = Q0, a teardrop with a corner at its foot, in the plane square to the path, in
coordinates (n, b) about the path point, n along the principal normal of the path and b along its
binormal (the unit tangent crossed with the principal normal) there: Q0 = Q3 = (0, −2),
Q1 = (8, 3), Q2 = (−8, 3). The curve leaves its foot to the n side and rises, rounds over the top
at b = 1.75 and comes down to its foot from the other side; it is 4.62 wide at most and encloses
an area of 12. So the section turns about the path as the path's Frenet frame does, with its
torsion. At the start the tangent is (3, 4, 0)/5, the principal normal n = (4, −3, 0)/5 and the
binormal b = (0, 0, −1), so the section's control points there are (0, 0, 2), (6.4, −4.8, −3),
(−6.4, 4.8, −3) and (0, 0, 2). The two ends of the pipe are flat, each the section there.
