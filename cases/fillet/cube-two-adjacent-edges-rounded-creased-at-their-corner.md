+++
title = "A cube with two adjacent edges rounded, their rounds creased where they meet"
tags = ["fillet", "corner", "cylinder", "planar", "analytic"]
origin = "OCCT blend/simple/P8"

[volume]
value = 983.5988157907700
exact = "10³ − 2·10·r²(1 − π/4) + (5/3 − π/2)r³, r = 2: the two removed prisms, each 10·r²(1 − π/4), overlap in ∫₀ʳ (r − √(r² − t²))² dt = (5/3 − π/2)r³"
tex = '10^3 - 2\cdot 10\,r^2\left(1 - \tfrac{\pi}{4}\right) + \left(\tfrac{5}{3} - \tfrac{\pi}{2}\right)r^3,\quad r = 2'

[area]
value = 580.5486677646163
exact = "3·10² − 2r²(1 − π/4) + (10 − r)² + 2·10(10 − r) + 2[(π/2)·r·10 − r²(π/2 − 1)], r = 2: the bottom face whole; the faces x = 0 and y = 10, each less one round's corner r²(1 − π/4); the top, trimmed on both sides, (10 − r)²; the faces x = 10 and y = 0, each cut at z = 10 − r, 10(10 − r); and two quarter cylinders each less the strip the other round takes"
tex = '3\cdot 10^2 - 2r^2\left(1-\tfrac{\pi}{4}\right) + (10-r)^2 + 2\cdot 10(10-r) + 2\left[\tfrac{\pi}{2}\,10\,r - r^2\left(\tfrac{\pi}{2} - 1\right)\right],\quad r = 2'
+++

A cube of side 10 between the origin and (10, 10, 10). Two of its edges meet at the corner
(10, 0, 10): the edge along y where the faces x = 10 and z = 10 meet, and the edge along x where the
faces y = 0 and z = 10 meet. Round both to radius 2.

The third edge at that corner, the upright one where x = 10 meets y = 0, is not rounded, so the two
rounds do not close the corner with a patch of their own. Each round is a quarter cylinder running
the whole length of its edge, tangent to both of the edge's faces, and neither is shortened by the
other: where they cross they meet in a sharp crease, the curve where the two cylinders, of equal
radius, intersect. The solid is the cube less what each round removes.

**As a set:**

$$[0,10]^3 \setminus \Big(\{\,x \ge 8,\ z \ge 8,\ (x-8)^2 + (z-8)^2 \ge 4\,\} \cup \{\,y \le 2,\ z \ge 8,\ (y-2)^2 + (z-8)^2 \ge 4\,\}\Big)$$
