+++
title = "A block less a B-spline cube touching it along an edge"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/B4, OCCT boolean/bcut_simple/D2"

[volume]
value = 1000
exact = "10³"
tex = '10^3'

[area]
value = 600
exact = "6 · 10²"
tex = '6\cdot 10^2'
+++

A block from (0, 10, 10) to (10, 20, 20), its faces planes. And a cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. Cut the cube from the block. The block touches the cube along one whole edge, the top edge at y = 10, and nowhere else: they share a line and no face. Nothing is removed: the result is the whole block.

**As a set:**

$$([0,10]\times[10,20]\times[10,20]) \setminus [0,10]^3$$
