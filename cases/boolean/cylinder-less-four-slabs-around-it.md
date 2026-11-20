+++
title = "A cylinder less four slabs in a grid around it"
tags = ["boolean", "cut", "cylinder", "analytic"]
origin = "OCCT tests/bugs/modalg_5/bug25614_cut21"

[volume]
value = 208.74167777251514
exact = "72π − 8a, a = (9/2)(arccos(1/3) − arcsin(1/3)) − 2√2 + 1 = 2.18162416: the cylinder 9π·8 less the eight corner pieces (four notches, each 2 thick, of area a), a = the part of the disk of radius 3 in the quadrant x ≥ 1, y ≥ 1"
tex = '72\pi - 8a,\quad a = \tfrac92\left(\arccos\tfrac13 - \arcsin\tfrac13\right) - 2\sqrt2 + 1'

[area]
value = 232.66600222953572
exact = "66π − 8ℓ + 4(2a + 4(2√2 − 1)), ℓ = 3(π/2 − 2 arctan(1/(2√2))) = 2.67336752: the cylinder's surface 48π + 18π, less its wall where each notch cuts it (ℓ wide, 2 high, four notches: 8ℓ), plus in each notch a floor and a ceiling of area a and two flat sides, each 2√2 − 1 long and 2 high"
tex = '66\pi - 8\ell + 4\left(2a + 4(2\sqrt2 - 1)\right)'
+++

A cylinder of radius 3 stands on the z axis from z = −3 to z = 5. Four square slabs, each 10 × 10 × 2, lie around it in a two-by-two grid with a gap of 2 between them: they fill z from 0 to 2 and, in x and y, the squares [1, 11] or [−11, −1] on each axis. Cut the slabs from the cylinder. What is left is one solid: the cylinder with a notch in its side at each of four places, between z = 0 and z = 2, the notches bounded by two flat walls meeting at a right angle, a floor and a ceiling, and the cross-shaped gap between the slabs still joining the part above to the part below.

**As a set:**

$$\{x^2 + y^2 \le 3^2,\ -3 \le z \le 5\} \setminus \{1 \le |x| \le 11,\ 1 \le |y| \le 11,\ 0 \le z \le 2\}$$
