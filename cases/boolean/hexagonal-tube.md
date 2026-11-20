+++
title = "A hexagonal tube"
tags = ["boolean", "cut", "prism", "planar", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPrimitive.py::testPrimitivePrism"

[volume]
value = 332.55375505322444
exact = "h·(3√3/2)(R² − r²) = 8·(3√3/2)(25 − 9) = 192√3"
tex = '8\cdot\tfrac{3\sqrt3}{2}\left(5^2 - 3^2\right) = 192\sqrt{3}'

[area]
value = 467.13843876330611
exact = "the two ends, 2·(3√3/2)(25 − 9) = 48√3, and twelve sides 8 high, six 5 wide and six 3 wide: 8(30 + 18) = 384"
tex = '48\sqrt3 + 8\,(6\cdot 5 + 6\cdot 3) = 384 + 48\sqrt{3}'
+++

A regular hexagonal prism standing on the xy plane, 8 high, its hexagon centered on the z axis with
its corners at distance 5 from it (so its sides are 5 long), one corner on the +x axis. A smaller
regular hexagonal prism of the same height, on the same axis, its corners at distance 3 from it, one
on the +x axis, with its sides parallel to the larger's, is cut away. Both span z = 0 to z = 8, so
the tube is open at both ends, with flat ring ends flush with the prisms'.

If you cut the inside with a prism, make it run past both ends.

**As a set:**

$$\bigl(H_5 \setminus H_3\bigr) \times [0, 8]$$

H_R is the regular hexagon centered on the origin with corners (R cos 60°k, R sin 60°k), k = 0..5.
