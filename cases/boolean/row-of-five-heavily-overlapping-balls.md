+++
title = "A row of five heavily overlapping balls"
tags = ["boolean", "union", "sphere", "analytic", "overlap"]
origin = "OCCT tests/bugs/modalg_5/bug24359"

[volume]
value = 225.14747350726852
exact = "π∫ρ(x)²dx with ρ² = 9 − (x − c)², c the nearest centre: two end zones, 2·∫₋₃^½(9 − x²)dx = 269.5/6, and three middle zones, 3·∫₋½^½(9 − x²)dx = 26.75; together 215/3, times π"
tex = '\pi\Bigl(2\int_{-3}^{1/2}(9 - x^2)\,dx + 3\int_{-1/2}^{1/2}(9 - x^2)\,dx\Bigr) = \tfrac{215\pi}{3}'

[area]
value = 188.49555921538759
exact = "the outside is a spherical zone of each ball, 2πR per unit of x: 2π·3·(3.5 + 1 + 1 + 1 + 3.5)"
tex = '2\pi\cdot 3\cdot 10 = 60\pi'
+++

Five equal balls of radius 3 with their centers in a row on the x axis, 1 apart: at (0, 0, 0),
(1, 0, 0), (2, 0, 0), (3, 0, 0) and (4, 0, 0). Each center lies inside every other ball, and the
middle of the row is inside all five at once. Unite them, one after another from the first.

Every point is as near to its closest center as to any, so the outside of the united solid, a
solid of revolution about the x axis from x = −3 to x = 7, is made of a spherical zone of each ball:
the first from x = −3 to x = ½, the middle three each 1 wide, between ½ and 3½ and the last from 3½ to
7. Neighbouring zones meet in circles of radius √(9 − ¼) at x = ½, 1½, 2½ and 3½, which are creases.

**As a set:**

$$\bigcup_{k=0}^{4} B_3\bigl((k, 0, 0)\bigr)$$
