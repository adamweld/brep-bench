+++
title = "Three quarters of a cone frustum, one cut flat moved in"
tags = ["edit", "move-face", "cone", "concave", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/J5"

[volume]
value = 961.7159018587498
exact = "234π + F(9) − F(3), F(r) = (r³/3)·acos(2/r) − (4/3)·r·w + (8/3)·ln(r + w), w = √(r² − 4): the frustum's 468π less its part with x > 0 and y < 2, a slice of radius r = 9 − z/2 losing πr²/2 less half the circular segment beyond the chord y = 2, whose area is r²·acos(2/r) − 2w"
tex = '234\pi + F(9) - F(3),\quad F(r) = \tfrac{r^3}{3}\arccos\tfrac{2}{r} - \tfrac{4}{3}rw + \tfrac{8}{3}\ln(r + w),\quad w = \sqrt{r^2 - 4}'

[area]
value = 703.831804354065
exact = "√5·(54π − G(9) + G(3)) + 96 + 2·(L(9) − L(3)) + (81π/2 + S(9)/2) + (9π/2 + S(3)/2), G(r) = (r²/2)·asin(2/r) + w, L(r) = rw/2 − 2·ln(r + w), S(r) = r²·acos(2/r) − 2w, w = √(r² − 4): the cone's side (√5/2 times the area it stands over, each slice's arc being 3π/2 − asin(2/r) of its circle); the flat in x = 0, whose slices are y from −r to 2; the moved flat in y = 2, whose slices are x from 0 to w; and the base and the top, each a disk less its part with x > 0 and y < 2"
tex = '\sqrt{5}\,\bigl(54\pi - G(9) + G(3)\bigr) + 96 + 2\bigl(L(9) - L(3)\bigr) + \tfrac{81\pi + S(9)}{2} + \tfrac{9\pi + S(3)}{2},\quad G = \tfrac{r^2}{2}\arcsin\tfrac{2}{r} + w,\ L = \tfrac{rw}{2} - 2\ln(r + w),\ S = r^2\arccos\tfrac{2}{r} - 2w'
+++

Three quarters of a cone frustum about the z axis, of radius 9 at z = 0 and 3 at z = 12 (its cone's
apex would be at z = 18): the part whose angle about the z axis, φ, measured from +x toward +y,
lies between 0° and 270°. Its faces are the cone; a bottom and a top, three quarters of disks of
radius 9 and 3; and two flat trapezoids, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the
half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the axis in a concave edge.

Then edit the solid directly: move the flat at φ = 0° 2 inward, against its outward normal −y, to
the plane y = 2. The flat gives up the slab it stood on, so the faces beside it are trimmed: the
cone, the bottom and the top lose their parts with x > 0 and y < 2, and the other flat, in x = 0,
runs on to y ≤ 2. The two flats meet along the line x = 0, y = 2, in a concave edge from the bottom
to the top. The axis itself is left in the part taken away.

So the result is the whole frustum less its part with x > 0 and y < 2.

**As a set:**

$$\{\, \rho \le 9 - \tfrac{z}{2},\ 0 \le z \le 12 \,\} \setminus \{\, x > 0,\ y < 2 \,\}$$
