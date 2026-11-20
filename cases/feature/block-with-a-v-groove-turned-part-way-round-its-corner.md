+++
title = "A block with a V-groove turned part way round its corner, ending blind"
tags = ["feature", "groove", "revolve", "blind", "cone", "planar", "analytic"]
origin = "OCCT feat/featrevol/A4, OCCT feat/featrevol/A5"

[volume]
value = 1361.4601836602552
exact = "1440 − (50/360)·2π·ρ̄·A = 1440 − (5/36)·2π·6·15 = 1440 − 25π"
tex = '1440 - \tfrac{50}{360}\cdot 2\pi\cdot 6\cdot 15 = 1440 - 25\pi'

[area]
value = 797.64565891837367
exact = "768 − 10π [the top's sector, (50/360)π(9² − 3²)] − 15 [the front face's triangle] + 15 [the groove's flat end, a triangle] + (10/3)π√34 [two cones, (5/36)·2π·(4.5 + 7.5)√34]"
tex = '768 - 10\pi - 15 + 15 + \tfrac{10}{3}\pi\sqrt{34}'
+++

A square block with a V-shaped groove turned about its vertical edge through a fixed angle, 50°, so
that the groove stops inside the block.

- The block: 0 ≤ x ≤ 12, 0 ≤ y ≤ 12, 0 ≤ z ≤ 10.
- The sketch: the triangle in the plane y = 0, on the block's front face, with corners (3, 0, 10),
  (9, 0, 10) and (6, 0, 5).
- Turn it about the z axis from the front face, towards +y, through 50°, and cut away what it sweeps.

The groove opens through the block's top and through the front face (a triangle), and ends in a flat
triangular wall inside the block, in the plane through the z axis at 50° from +x.

**As a set:** with $\rho = \sqrt{x^2 + y^2}$ and $\varphi$ the angle about the z axis from +x,

$$\bigl([0,12]^2 \times [0,10]\bigr) \setminus \{\,5 + \tfrac{5}{3}|\rho - 6| \le z \le 10,\ 0 \le \varphi \le 50^\circ\,\}$$
