+++
title = "A B-spline cube notched along an edge by a block"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/B5"

[volume]
value = 840
exact = "10³ − 10·4·4"
tex = '10^3 - 10\cdot 4\cdot 4'

[area]
value = 568
exact = "6·10² − 2·10·4 + 2·10·4 − 2·4²: the notch's two sides replace two strips, and each end face loses a 4 × 4 square"
tex = '6\cdot 10^2 - 2\cdot 4^2'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 0) to (10, 4, 4), its faces planes. Cut the block from the cube. The block's long side lies along the cube's bottom edge at y = 0, z = 0, from end to end, so its two ends lie in the cube's end faces and two of its sides in the cube's faces y = 0 and z = 0. A square notch of 4 by 4 is left along the edge.

**As a set:**

$$[0,10]^3 \setminus ([0,10]\times[0,4]\times[0,4])$$
