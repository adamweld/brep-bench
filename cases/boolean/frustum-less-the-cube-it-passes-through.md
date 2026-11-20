+++
title = "A frustum less the cube it passes through"
tags = ["boolean", "cut", "analytic", "cone", "planar", "pieces"]
origin = "OCCT tests/boolean/boptuc_simple/ZF8"

[volume]
value = 43.10265120725196
exact = "two frusta, πh/3·(a² + ab + b²) with h = 2: radii 5/2 and 11/5, and radii 13/10 and 1"
tex = '\tfrac{2\pi}{3}\left(\tfrac{25}{4} + \tfrac{11}{2} + \tfrac{121}{25}\right) + \tfrac{2\pi}{3}\left(\tfrac{169}{100} + \tfrac{13}{10} + 1\right)'

[area]
value = 87.76549238308954
exact = "two cone walls, π(a + b)·√(h² + (a − b)²), and four flat ends, π times the squares of the radii 5/2, 11/5, 13/10 and 1"
tex = '\pi\left(\tfrac{5}{2} + \tfrac{11}{5}\right)\sqrt{4 + \tfrac{9}{100}} + \pi\left(\tfrac{13}{10} + 1\right)\sqrt{4 + \tfrac{9}{100}} + \pi\left(\tfrac{25}{4} + \tfrac{121}{25} + \tfrac{169}{100} + 1\right)'
+++

A cube of edge 6 with x, y and z from 0 to 6, and a frustum of a cone with its axis the vertical line through (3, 3), from z = −2 to z = 8, of radius 5/2 at the bottom and 1 at the top (its radius falls by 3/20 for every unit of rise). The frustum is taller than the cube and passes out through both its end faces; at the cube's base its radius is 11/5 and at its top 13/10, so it stays inside the cube's sides, within 3 of the axis. Cut the cube away from the frustum.

What is left is the two ends of the frustum that stand out of the cube: a lower one from z = −2 to z = 0, and an upper one from z = 6 to z = 8, two separate pieces. Build it all as one result and measure it whole.

**As a set:**

$$\{\,\rho \le \tfrac{5}{2} - \tfrac{3}{20}(z + 2),\ -2 \le z \le 8\,\} \setminus \bigl([0,6]\times[0,6]\times[0,6]\bigr)$$

where ρ is the distance from the vertical line through (3, 3).
