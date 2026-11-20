+++
title = "A B-spline cube less a block touching it at a corner"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/D3"

[volume]
value = 1000
exact = "10³"
tex = '10^3'

[area]
value = 600
exact = "6 · 10²"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (10, 10, 10) to (20, 20, 20), its faces planes. Cut the block from the cube. The block touches the cube at the single point (10, 10, 10), the cube's far corner, and nowhere else. Nothing is removed: the result is the whole cube.

**As a set:**

$$[0,10]^3 \setminus ([10,20]\times[10,20]\times[10,20])$$
