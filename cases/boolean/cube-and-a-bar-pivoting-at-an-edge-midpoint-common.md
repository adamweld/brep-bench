+++
title = "A cube and a bar turned diagonally from the midpoint of one of its edges, their common part"
tags = ["boolean", "common", "analytic", "planar", "coincident", "vertex-on-plane"]
origin = "OCCT boolean/bcommon_simple/G9, OCCT boolean/bcommon_simple/H2"

[volume]
value = 136.42135623730951
exact = "10·(10√2 − 1/2): a prism 10 high on the pentagon with corners (5, 0), (5 + √2, 0), (10, 5 − √2), (10, 5 + √2), (5 − √2/2, √2/2), of area 10√2 − 1/2"
tex = '10\left(10\sqrt2 - \tfrac12\right) = 100\sqrt2 - 5'

[area]
value = 211.13203435596426
exact = "two pentagons of 10√2 − 1/2 and five sides 10 high on the pentagon's sides √2, 5√2 − 2, 2√2, 5√2 + 1 and 1, which total 13√2: 20√2 − 1 + 130√2"
tex = '2\left(10\sqrt2 - \tfrac12\right) + 10\cdot 13\sqrt2 = 150\sqrt2 - 1'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a bar 2 wide, 9 long and 10 high, standing on the xy plane, so that its top lies in the cube's top plane and its bottom in the cube's bottom plane. On the xy plane the bar's plan is the rectangle with corners (5 + √2/2, −√2/2), (5 − √2/2, √2/2), (5 + 4√2, 5√2) and (5 + 5√2, 4√2): one of its short ends is centred on (5, 0), the midpoint of the cube's bottom front edge, and its long sides run from there along the direction (1, 1), the diagonal direction across the cube. Make the solid common to both.

The bar's end sticks out of the cube's face y = 0 around the pivot point, and its far end lies outside the cube's face x = 10, which the bar crosses between y = 5 − √2 and y = 5 + √2. None of the cube's corners is inside the bar. The common part is a prism 10 high on the pentagon with corners (5, 0), (5 + √2, 0), (10, 5 − √2), (10, 5 + √2) and (5 − √2/2, √2/2): its sides lie in the cube's faces y = 0 and x = 10, in the bar's two long sides and in the bar's end side x + y = 5.

**As a set:**

$$[0,10]^3 \cap \{\,0 \le x + y - 5 \le 9\sqrt2,\ |y - x + 5| \le \sqrt2\,\}$$
