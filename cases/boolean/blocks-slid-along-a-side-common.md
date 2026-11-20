+++
title = "Two equal cubes, one slid along a side, their common part"
tags = ["boolean", "common", "analytic", "planar", "coincident"]
origin = "FreeCAD tests/src/Mod/Part/App/FeaturePartCommon.cpp::testIntersecting"

[volume]
value = 600
exact = "6·10·10"
tex = '6\cdot 10\cdot 10'

[area]
value = 440
exact = "2(6·10 + 6·10 + 10·10)"
tex = '2\,(6\cdot 10 + 6\cdot 10 + 10\cdot 10)'
+++

Two cubes of side 10: the first from the origin to (10, 10, 10), the second from (4, 0, 0) to (14, 10, 10), the first slid 4 along x. They overlap in a block from x = 4 to x = 10, and their faces y = 0, y = 10, z = 0 and z = 10 lie in common planes. Make the part common to both.

The result is the block from (4, 0, 0) to (10, 10, 10).

**As a set:**

$$[0,10]^3 \cap \bigl([4,14]\times[0,10]^2\bigr)$$
