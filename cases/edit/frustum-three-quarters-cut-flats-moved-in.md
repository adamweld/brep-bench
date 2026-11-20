+++
title = "Three quarters of a cone frustum, both cut flats moved in"
tags = ["edit", "move-face", "cone", "concave", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/J6"

[volume]
value = 772.7327823074821
exact = "468π − 2(H(9) − H(3)), H(r) = (r³/3)·asin(2/r) + (4/3)·r·w − (8/3)·ln(r + w) + πr³/12 + 4r, w = √(r² − 4): the frustum less its part with x > −2 and y < 2, each slice of radius r = 9 − z/2 losing r²·asin(2/r) + 2w + πr²/4 + 4 (the quarter disk past the corner (−2, 2) with the two strips beside it, the corner lying inside every slice)"
tex = '468\pi - 2\bigl(H(9) - H(3)\bigr),\quad H(r) = \tfrac{r^3}{3}\arcsin\tfrac{2}{r} + \tfrac{4}{3}rw - \tfrac{8}{3}\ln(r + w) + \tfrac{\pi r^3}{12} + 4r,\quad w = \sqrt{r^2 - 4}'

[area]
value = 664.2661091146196
exact = "√5·(54π − 2(G(9) − G(3))) + 4·(12 + L(9) − L(3)) + (81π − E(9)) + (9π − E(3)), G(r) = (r²/2)·asin(2/r) + w, L(r) = rw/2 − 2·ln(r + w), E(r) = r²·asin(2/r) + 2w + πr²/4 + 4, w = √(r² − 4): the cone's side (√5/2 times the area it stands over, each slice's arc being 3π/2 − 2·asin(2/r) of its circle), the two flats in x = −2 and y = 2, each with slices 2 + w long, and the base and top, each a disk less E(r)"
tex = '\sqrt{5}\bigl(54\pi - 2(G(9) - G(3))\bigr) + 4\bigl(12 + L(9) - L(3)\bigr) + 90\pi - E(9) - E(3),\quad E = r^2\arcsin\tfrac{2}{r} + 2w + \tfrac{\pi r^2}{4} + 4'
+++

Three quarters of a cone frustum about the z axis, of radius 9 at z = 0 and 3 at z = 12 (its cone's
apex would be at z = 18): the part whose angle about the z axis, φ, measured from +x toward +y,
lies between 0° and 270°. Its faces are the cone; a bottom and a top, three quarters of disks of
radius 9 and 3; and two flat trapezoids, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the
half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the axis in a concave edge.

Then edit the solid directly: move each of the two flats 2 inward, against its outward normal: the
one at φ = 0° from y = 0 to the plane y = 2, and the one at φ = 270°, whose outward normal is +x,
from x = 0 to the plane x = −2. The faces beside them are trimmed, and each flat runs on to meet the
other, in a concave edge along the line x = −2, y = 2 from the bottom to the top. The cone, the
bottom and the top lose the parts they had with x > −2 and y < 2.

So the result is the whole frustum less its part with x > −2 and y < 2.

**As a set:**

$$\{\, \rho \le 9 - \tfrac{z}{2},\ 0 \le z \le 12 \,\} \setminus \{\, x > -2,\ y < 2 \,\}$$
