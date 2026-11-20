+++
title = "A cube and a wide bar turned across its diagonal from just outside a corner, their common part"
tags = ["boolean", "common", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcommon_simple/H4, OCCT boolean/bcommon_simple/H5"

[volume]
value = 668.5281374238570
exact = "10·(60√2 − 18): a prism 10 high on the hexagon with corners (0, 0), (3√2, 0), (10, 10 − 3√2), (10, 10), (10 − 3√2, 10), (0, 3√2), of area 100 − (10 − 3√2)² = 60√2 − 18"
tex = '10\left(100 - (10 - 3\sqrt2)^2\right) = 600\sqrt2 - 180'

[area]
value = 466.2539674441618
exact = "two hexagons of 60√2 − 18 and six sides 10 high on the hexagon's sides 3√2, √2(10 − 3√2), 3√2, 3√2, √2(10 − 3√2), 3√2, which total 32√2 − 12: 120√2 − 36 + 320√2 − 120"
tex = '2\left(60\sqrt2 - 18\right) + 10\left(32\sqrt2 - 12\right) = 440\sqrt2 - 156'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a bar 6 wide, 20 long and 10 high, standing on the xy plane, so that its top lies in the cube's top plane and its bottom in the cube's bottom plane. On the xy plane the bar's plan is the rectangle one of whose short ends is centred on (−1, −1), just outside the cube's corner (0, 0), and whose long sides run from there along the direction (1, 1), along the cube's plan diagonal: its corners are (−1 + 3√2/2, −1 − 3√2/2) and (−1 − 3√2/2, −1 + 3√2/2) at that end, and those two moved by (10√2, 10√2) at the other. The bar's plan is the strip |y − x| ≤ 3√2 between the lines x + y = −2 and x + y = −2 + 20√2. Make the solid common to both.

Both end sides of the bar are clear of the cube, so the bar crosses the cube from the corner (0, 0) to the corner (10, 10), both inside it; the cube's other two bottom corners, (10, 0) and (0, 10), are outside it. The common part is a prism 10 high on the hexagon with corners (0, 0), (3√2, 0), (10, 10 − 3√2), (10, 10), (10 − 3√2, 10) and (0, 3√2): the cube's plan less two triangles, which the bar's long sides cut off the corners (10, 0) and (0, 10).

**As a set:**

$$[0,10]^3 \cap \{\,|y - x| \le 3\sqrt2\,\}$$
