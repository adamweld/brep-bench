+++
title = "An oblique prism with a curved spline wall, its sloping top moved in and its side flat moved out along the wall"
tags = ["edit", "move-face", "bspline", "extrusion", "oblique", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/M1"

[volume]
value = 18.0
exact = "8·∫₋₁² (1 − x²/4) dx = 8·(9/4) = 18: an oblique extrusion's volume is its length 8 up z times the section's area, and the section, taken level across the slope, is the region between the curve y = x²/4 and the line y = 1, for x from −1 to 2"
tex = '8\int_{-1}^{2}\left(1 - \tfrac{x^2}{4}\right)dx = 8\cdot\tfrac{9}{4} = 18'

[area]
value = 72.72520797970178
exact = "2·(9/4) + 6 + 30 + 2√41 + √29 + (25/2)(asinh(4/5) + asinh(2/5)): the two ends, each the level section; the flat side in x = −1, a parallelogram of sides 3/4 and the slope's vector (0, 6, 8), area (3/4)·8; the sloping top, a parallelogram of sides 3 along x and 10 up the slope; and the curved wall, ∫₋₁² |c′ × V| dx with c′ = (1, x/2, 0) and V = (0, 6, 8), |c′ × V| = 4√(x² + 25/4)"
tex = '\tfrac{9}{2} + 6 + 30 + 2\sqrt{41} + \sqrt{29} + \tfrac{25}{2}\left(\operatorname{asinh}\tfrac{4}{5} + \operatorname{asinh}\tfrac{2}{5}\right)'
+++

An oblique prism: the region in the plane z = 0 bounded by three edges, swept along the vector
V = (0, 6, 8), so that its far end, in the plane z = 8, is the same region shifted 6 up in y. The
region is bounded by a Bézier curve of degree 4 (one span of a B-spline of degree 4, knots 0 and 1
each repeated five times) from (0, 0) to (4, 4) with control points (0, 0), (1, 0), (2, 2/3), (3, 2)
and (4, 4), which is the parabola y = x²/4, x from 0 to 4; a straight edge from (4, 4) to (0, 4);
and a straight edge from (0, 4) to (0, 0). Its faces are the curved wall; the sloping top, the
plane through the edge y = 4, z = 0 and the direction V, that is y − 3z/4 = 4, whose outward normal
is (0, 4, −3)/5; the flat side in the plane x = 0, which contains V and whose outward normal is
−x; and the two ends, z = 0 and z = 8.

Then edit the solid directly, moving two faces at once: the sloping top 12/5 inward, against its
outward normal, which moves its plane to y − 3z/4 = 1 (a normal distance of 12/5 is a shift of 3
along y), and the side flat 1 outward, to the plane x = −1. The curved wall is cut off by the new
top, where its parabola reaches y − 3z/4 = 1, at x = 2 on every level, and runs on past (0, 0) along
its own parabola to meet the new side, at x = −1.

**As a set:**

$$\{\, -1 \le x \le 2,\ x^2/4 \le y - \tfrac{3z}{4} \le 1,\ 0 \le z \le 8 \,\}$$
