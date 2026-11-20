+++
title = "A prism with a curved spline wall, its top flat moved in past the wall's middle"
tags = ["edit", "move-face", "bspline", "extrusion", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/L2"

[volume]
value = 13.333333333333334
exact = "10·∫₀² (1 − x²/4) dx = 10·(4/3) = 40/3: the prism's section is the region between the wall's curve y = x²/4 and the line y = 1, for x from 0 to 2"
tex = '10\int_0^2 \left(1 - \tfrac{x^2}{4}\right)dx = 10\cdot\tfrac{4}{3} = \tfrac{40}{3}'

[area]
value = 55.62253816059305
exact = "2·(4/3) + 10 + 20 + 10·(√2 + ln(1 + √2)): the two ends, each the section; the flat in x = 0 (y from 0 to 1, 10 long); the moved flat in y = 1 (x from 0 to 2, 10 long); and the curved wall, 10 times the length of the curve y = x²/4 from x = 0 to 2, ∫₀² √(1 + x²/4) dx = √2 + asinh 1"
tex = '2\cdot\tfrac{4}{3} + 10 + 20 + 10\left(\sqrt{2} + \ln(1 + \sqrt{2})\right)'
+++

A prism of length 10 standing on the xy plane, from z = 0 to z = 10, whose section is the region in
the plane z = 0 bounded by three edges: a cubic Bézier curve (one span of a B-spline of degree 3,
knots 0 and 1 each repeated four times) from (0, 0) to (4, 4) with control points (0, 0),
(4/3, 0), (8/3, 4/3) and (4, 4), which is the parabola y = x²/4, x from 0 to 4; a straight edge
from (4, 4) to (0, 4); and a straight edge from (0, 4) to (0, 0). Its faces are the curved wall,
which is the cylinder over the parabola; the flat top, in the plane y = 4, whose outward normal is
+y; the flat side in the plane x = 0; and the two ends, z = 0 and z = 10.

Then edit the solid directly: move the top flat 3 inward, against its outward normal, to the plane
y = 1. The faces beside it are trimmed: the curved wall is cut off where its parabola reaches
y = 1, at x = 2, and the flat side in x = 0 is cut down to y ≤ 1. The solid is left with the
curved wall from (0, 0) to (2, 1), the new top in y = 1 from x = 2 back to x = 0, and the flat
side from (0, 1) down to (0, 0); the right-hand end of the old section, at (4, 4), is gone.

**As a set:**

$$\{\, 0 \le x \le 2,\ x^2/4 \le y \le 1,\ 0 \le z \le 10 \,\}$$
