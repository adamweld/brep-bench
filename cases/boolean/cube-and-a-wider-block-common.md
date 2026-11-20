+++
title = "A cube and a wider block flush with it, their common part"
tags = ["boolean", "common", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcommon_simple/A2"

[volume]
value = 1000
exact = "the cube itself: 10·10·10"
tex = '10^3'

[area]
value = 600
exact = "the cube's six faces: 6·100"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block from (-5, -5, 0) to (15, 15, 10). Make the solid common to both.

The block's footprint holds the cube's, and the two are the same height, so the cube's top lies in the block's top and its bottom in the block's bottom. The cube's four sides lie inside the block. All of the cube is in the block, so the common part is the cube.

**As a set:**

$$[0,10]^3 \cap \bigl([-5,15]^2\times[0,10]\bigr)$$
