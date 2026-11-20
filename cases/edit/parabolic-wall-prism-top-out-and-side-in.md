+++
title = "A prism with a curved spline wall, its top flat moved out along the wall and its side flat moved in"
tags = ["edit", "move-face", "bspline", "extrusion", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/L3"

[volume]
value = 270.8333333333333
exact = "10·∫₁⁶ (9 − x²/4) dx = 10·(45 − 215/12) = 1625/6: the section is the region between the curve y = x²/4 and the line y = 9, for x from 1 to 6"
tex = '10\int_1^6 \left(9 - \tfrac{x^2}{4}\right)dx = 10\left(45 - \tfrac{215}{12}\right) = \tfrac{1625}{6}'

[area]
value = 294.3171728696932
exact = "2·(325/12) + 10·(35/4) + 10·5 + 10·(P(3) − P(1/2)), P(u) = u·√(1 + u²) + asinh u: the two ends, each the section; the flat side in x = 1 (y from 1/4 to 9); the top in y = 9 (x from 1 to 6); and the curved wall, 10 times the length of the curve y = x²/4 from x = 1 to 6, ∫ √(1 + x²/4) dx = P(x/2)"
tex = '\tfrac{325}{6} + \tfrac{175}{2} + 50 + 10\bigl(P(3) - P(\tfrac{1}{2})\bigr),\quad P(u) = u\sqrt{1 + u^2} + \operatorname{asinh} u'
+++

A prism of length 10 standing on the xy plane, from z = 0 to z = 10, whose section is the region in
the plane z = 0 bounded by three edges: a cubic Bézier curve (one span of a B-spline of degree 3,
knots 0 and 1 each repeated four times) from (0, 0) to (4, 4) with control points (0, 0),
(4/3, 0), (8/3, 4/3) and (4, 4), which is the parabola y = x²/4, x from 0 to 4; a straight edge
from (4, 4) to (0, 4); and a straight edge from (0, 4) to (0, 0). Its faces are the curved wall,
which is the cylinder over the parabola; the flat top, in the plane y = 4, whose outward normal is
+y; the flat side in the plane x = 0, whose outward normal is −x; and the two ends, z = 0 and z = 10.

Then edit the solid directly, moving two faces at once: the top flat 5 outward, to the plane y = 9,
and the side flat 1 inward, to the plane x = 1. The curved wall runs on beyond (4, 4) along its own
parabola to meet the new top, at (6, 9), and is cut off at the new side, at (1, 1/4). The solid's
section is now bounded by the parabola from (1, 1/4) to (6, 9), the new top from (6, 9) back to
(1, 9), and the new side from (1, 9) down to (1, 1/4).

**As a set:**

$$\{\, 1 \le x \le 6,\ x^2/4 \le y \le 9,\ 0 \le z \le 10 \,\}$$
