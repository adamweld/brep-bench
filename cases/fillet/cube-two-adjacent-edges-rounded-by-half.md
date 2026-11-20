+++
title = "A cube with two adjacent edges rounded by half its side"
tags = ["fillet", "corner", "miter", "cylinder", "planar", "analytic"]
origin = "OCCT tests/bugs/modalg_7/bug25478_1"

[volume]
value = 904.682874182695
exact = "1000 − 2·250(1 − π/4) + (625/3 − 125π/2) = 2125/3 + 125π/2: each round removes 250(1 − π/4), and the two overlap in ∫₀⁵ (5 − √(25 − w²))² dw = 625/3 − 125π/2"
tex = '1000 - 2\cdot 250\left(1 - \tfrac{\pi}{4}\right) + \left(\tfrac{625}{3} - \tfrac{125\pi}{2}\right) = \tfrac{2125}{3} + \tfrac{125\pi}{2}'

[area]
value = 542.809724509617
exact = "425 + 75π/2: the faces z = 0 (100), x = 0 and y = 0 (75 + 25π/4 each), x = 10 and y = 10 (50 each), the top (25), and the two rounds (25π/2 + 25 each)"
tex = '425 + \tfrac{75\pi}{2}'
+++

A cube of side 10 between the origin and (10, 10, 10). Round its two top edges that meet at the
corner (10, 10, 10): the edge along x where the faces y = 10 and z = 10 meet, and the edge along
y where x = 10 and z = 10 meet. The radius is 5, half the side, so each round, a quarter
cylinder, reaches to the middle of the faces on either side of its edge, and the two rounds meet
along the top.

The round of the edge along x is the quarter cylinder about the line y = 5, z = 5, tangent to the
face y = 10 along z = 5 and to the face z = 10 along y = 5; the round of the edge along y is the
quarter cylinder about the line x = 5, z = 5, tangent to x = 10 along z = 5 and to z = 10 along
x = 5. At the corner (10, 10, 10) the rounds are neither blended nor capped: each runs on until
it meets the other, along the curve where the two cylinders cross. As a composition, the solid is
the cube rounded on the one edge and the cube rounded on the other, in common.

**As a set:**

$$[0,10]^3 \setminus \Bigl(\{\,y \ge 5,\ z \ge 5,\ (y-5)^2 + (z-5)^2 \ge 25\,\} \cup \{\,x \ge 5,\ z \ge 5,\ (x-5)^2 + (z-5)^2 \ge 25\,\}\Bigr)$$
