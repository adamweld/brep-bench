+++
title = "A half cylinder bored off-axis across its flat"
tags = ["boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT boolean/cells_test/I1, OCCT boolean/cells_test/I2"

[volume]
value = 233.28221562184605
exact = "8·(25π/2 − (8π/3 + √3)): the small circle (r = 2, centre 1 off the flat) has 8π/3 + √3 of its disc on the cylinder's side"
tex = '8\left(\tfrac{25\pi}{2} - \tfrac{8\pi}{3} - \sqrt3\right)'

[area]
value = 303.29209040453346
exact = "2·A + 5π·8 + (10 − 2√3)·8 + (8π/3)·8, A = 25π/2 − 8π/3 − √3"
tex = '2A + 5\pi\cdot 8 + \left(10 - 2\sqrt3\right)\cdot 8 + \tfrac{8\pi}{3}\cdot 8,\quad A = \tfrac{25\pi}{2} - \tfrac{8\pi}{3} - \sqrt3'
+++

A cylinder of radius 5 and height 8, its axis the z axis, from z = 0 to z = 8, split along the plane y = 0 through its axis; keep the half with y ≥ 0. A second cylinder of radius 2 stands parallel to the first, its axis through (0, 1), so that it crosses the flat y = 0. Bore it right through the half cylinder, past both ends. What is left is the half cylinder less the bore: the bore leaves a notch in the flat face, and its wall, 240° of the small cylinder, runs from end to end of the half cylinder.

**As a set:**

$$\{\,\rho \le 5,\ y \ge 0,\ 0 \le z \le 8\,\} \setminus \{\,x^2 + (y - 1)^2 \le 2^2\,\}$$
