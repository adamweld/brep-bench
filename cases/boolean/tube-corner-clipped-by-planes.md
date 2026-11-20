+++
title = "A tube corner clipped by six planes"
tags = ["boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT boolean/volumemaker/C8, OCCT boolean/volumemaker/D3, OCCT boolean/volumemaker/H2, OCCT boolean/volumemaker/H4"

[volume]
value = 61.539724033829046
exact = "6A + ½∬x dA, A = 4π + 12 − 25·acos(4/5), ∬x dA = 64/3: the section is the quarter of the tube's ring in the square 0 ≤ x, y ≤ 4"
tex = '6A + \tfrac12\iint x\,dA,\quad A = 4\pi + 12 - 25\arccos\tfrac45,\quad \iint x\,dA = \tfrac{64}{3}'

[area]
value = 119.74663459443683
exact = "A + (√5/2)A + 6 + 31/4 + 24 + 81/4 + (9π + 9/2) + 5(6Δ + 1/2), Δ = asin(4/5) − acos(4/5)"
tex = 'A + \tfrac{\sqrt5}{2}A + 6 + \tfrac{31}{4} + 24 + \tfrac{81}{4} + \left(9\pi + \tfrac92\right) + 5\left(6\Delta + \tfrac12\right),\quad \Delta = \arcsin\tfrac45 - \arccos\tfrac45'
+++

The part of a tube that lies in the corner square 0 ≤ x ≤ 4, 0 ≤ y ≤ 4, between the cylinders of radius 3 and 5 about the z axis, standing on the plane z = 0 and topped by the inclined plane z = 6 + x/2. It is bounded by six planes (x = 0, x = 4, y = 0, y = 4, z = 0 and the inclined top) and the two cylinders. The inner cylinder is whole, a quarter turn of it; the outer one is reached only in a narrow strip between its points at x = 4 (angle 36.87° from +x) and y = 4 (53.13°).

**As a set:**

$$\{\,3 \le \rho \le 5,\ 0 \le x \le 4,\ 0 \le y \le 4,\ 0 \le z \le 6 + \tfrac{x}{2}\,\}$$
