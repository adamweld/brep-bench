+++
title = "A box with the four edges round its top face rounded"
tags = ["fillet", "corner", "analytic"]
origin = "FreeCAD src/Mod/Part/parttests/TopoShapeTest.py::testTopoShapeMakeFillet"

[volume]
value = 1151.5634100912841
exact = "20·10·4 + ∫₀²(20 − 2δ)(10 − 2δ)dt, δ = 2 − √(4 − t²); with ∫δ = r²(1 − π/4), ∫δ² = (5/3 − π/2)r³: = 20·10·6 − 2·30·r²(1 − π/4) + 4(5/3 − π/2)r³, r = 2"
tex = 'V = 20\cdot 10\cdot 4 + \int_0^2 (20 - 2\delta)(10 - 2\delta)\,dt,\quad \delta = 2 - \sqrt{4 - t^2}'

[area]
value = 706.2300767579509
exact = "(20−2r)(10−2r) + 200 + 2·30·4 + 2r[10·π/2 − 2r(π/2 − 1)] + 2r[20·π/2 − 2r(π/2 − 1)], r = 2"
tex = '(20-2r)(10-2r) + 200 + 2\cdot 30\cdot 4 + 2r\bigl[10\tfrac{\pi}{2} - 2r(\tfrac{\pi}{2} - 1)\bigr] + 2r\bigl[20\tfrac{\pi}{2} - 2r(\tfrac{\pi}{2} - 1)\bigr],\quad r = 2'
+++

A box from the origin to (20, 10, 6). Round the four edges around its top face z = 6, each with radius 2; leave the four vertical edges and the four bottom edges sharp.

Each rounded edge is a quarter cylinder of radius 2, tangent to the top face and to its side face (the round of the edge at x = 20, z = 6 has its axis along y, at x = 18, z = 4). The vertical edges stay sharp, so at each top corner two such quarter cylinders of equal radius meet at right angles and are mitred: they run into each other along a curve in the vertical plane through the box's corner and the top face's corner, and there is no corner patch. The side faces are plane up to z = 4; the top face is the rectangle from (2, 2, 6) to (18, 8, 6).

**As a set:**

$$[0,20]\times[0,10]\times[0,6] \cap \{\,\min(x,\ 20-x,\ y,\ 10-y) \ge \delta(z)\,\},\quad \delta(z) = \begin{cases} 0 & z \le 4 \\ 2 - \sqrt{4 - (z-4)^2} & z \ge 4 \end{cases}$$
