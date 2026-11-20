+++
title = "A square bar turned round a rectangular loop with round corners"
tags = ["sweep", "planar-path", "corner", "round", "closed-path", "analytic"]
origin = "OCCT pipe/specific/K1"

[volume]
value = 4993.6460032938
exact = "2a·[4a(w + h) + (π − 4)a²], a = 3, w = 40, h = 30: = 4824 + 54π. The bar occupies |z| ≤ 3 over the band of points within 3 of the rectangle's outline: the square turned about a corner sweeps a quarter disk, so the band is the rectangle's outer offset, rounded at its corners (area wh + 2a(w + h) + πa²) less its inner offset, sharp ((w − 2a)(h − 2a))"
tex = '2a\left[4a(w + h) + (\pi - 4)a^2\right] = 4824 + 54\pi'

[area]
value = 3313.6460032938
exact = "3144 + 54π: the top and the bottom, each the band, 804 + 9π; the outer wall, 6(2(w + h) + 2πa) = 840 + 36π; the inner wall, 6·2((w − 2a) + (h − 2a)) = 696"
tex = '2\left[4a(w + h) + (\pi - 4)a^2\right] + 6\left[2(w + h) + 2\pi a\right] + 6\cdot 2\left[(w - 2a) + (h - 2a)\right] = 3144 + 54\pi'
+++

A closed pipe of square section, swept along a closed path of four straight runs, the section
carried round each corner by turning it about the corner.

The path is the rectangle in the plane z = 0 with corners (0, 0, 0), (40, 0, 0), (40, 30, 0) and
(0, 30, 0), taken from (20, 0, 0) along +x, once round, and closed.

The section is the square of side 6, centered on the path and square to it, with two sides
parallel to the z axis: at (20, 0, 0) it has corners (20, 3, 3), (20, −3, 3), (20, −3, −3) and
(20, 3, −3). Each run is the square swept straight along it, ending flat in the plane through the
corner square to the run. At each corner the two runs overlap on the inside of the turn, and the
gap they leave outside is filled by the square turned about the vertical line through the corner,
a quarter turn from square to one run to square to the next, which sweeps a quarter of a
cylinder of radius 3 and height 6. The bar closes on itself with no ends, a flat ring: flat top
and bottom at z = ±3, an outer wall rounded at the four corners, an inner wall sharp at them.

**As a set:**

$$\{\,(x, y, z) : |z| \le 3,\ d\bigl((x, y), \gamma\bigr) \le 3\,\},\quad \gamma = \partial\bigl([0, 40] \times
[0, 30]\bigr)$$
