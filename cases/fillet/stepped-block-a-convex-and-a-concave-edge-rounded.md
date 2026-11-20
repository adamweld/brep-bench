+++
title = "A stepped block with a convex and a concave edge along its length rounded to different radii"
tags = ["fillet", "concave", "planar", "cylinder", "analytic", "refine"]
origin = "FreeCAD tests/src/Mod/Part/App/FeatureFillet.cpp::testOtherEdges"

[volume]
value = 18807.942495882689
exact = "(625 − (1 − π/4)·16 + (1 − π/4)·25)·30 = (634 − 9π/4)·30 = 19020 − 135π/2"
tex = '\left(634 - \tfrac{9\pi}{4}\right)\cdot 30 = 19020 - \tfrac{135\pi}{2}'

[area]
value = 5037.977841293468
exact = "2(634 − 9π/4) + 30(112 + 9π/2) = 4628 + 261π/2: the two ends and the section's outline, 130 less 4(2 − π/2) less 5(2 − π/2), run 30"
tex = '2\left(634 - \tfrac{9\pi}{4}\right) + 30\left(112 + \tfrac{9\pi}{2}\right) = 4628 + \tfrac{261\pi}{2}'
+++

A stepped block made of two boxes joined, its coplanar faces merged, then two of its edges rounded. The boxes: the base from the origin to (40, 30, 10), and on it the upper block from (10, 0, 10) to (25, 30, 25), flush with the base at y = 0 and y = 30. Join them, and merge the two faces in the plane y = 0 into one face, and the two in y = 30 into one. The solid's section in a plane y = c is the polygon through (x, z) = (0, 0), (40, 0), (40, 10), (25, 10), (25, 25), (10, 25), (10, 10), (0, 10).

Round two edges that run along y: the convex edge through (x, z) = (10, 25), the top corner of the upper block's left side, with radius 4; and the concave edge through (25, 10), where the upper block's right side meets the base's top, with radius 5 (which fills the corner with material). Each round runs the whole 30 along y and ends on the faces y = 0 and y = 30. No other edge is touched.

**As a set:**

The section polygon in (x, z) with its corner at (10, 25) cut by a quarter circle of radius 4 about (14, 21) and its corner at (25, 10) filled up to a quarter circle of radius 5 about (30, 15), extruded from y = 0 to y = 30.
