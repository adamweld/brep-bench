+++
title = "Two balls tens of metres across, united"
tags = ["boolean", "union", "sphere", "analytic", "large", "scale"]
origin = "brep-conformance 1.0.0, booleans/scale/microns-to-metres-spheres"

[volume]
value = 765950208875225.6
exact = "4π(r₁³ + r₂³)/3 − πh₁²(3r₁ − h₁)/3 − πh₂²(3r₂ − h₂)/3, r₁ = 50000, r₂ = 40000, h₁ = 60000/7, h₂ = 80000/7"
tex = '\tfrac{4}{3}\pi\left(r_1^3 + r_2^3\right) - \tfrac{\pi}{3}h_1^2(3r_1 - h_1) - \tfrac{\pi}{3}h_2^2(3r_2 - h_2),\quad r_1 = 50000,\ r_2 = 40000,\ h_1 = \tfrac{60000}{7},\ h_2 = \tfrac{80000}{7}'

[area]
value = 45957012532.51354
exact = "4π(r₁² + r₂²) − 2π(r₁h₁ + r₂h₂), r₁ = 50000, r₂ = 40000, h₁ = 60000/7, h₂ = 80000/7"
tex = '4\pi\left(r_1^2 + r_2^2\right) - 2\pi\left(r_1 h_1 + r_2 h_2\right),\quad r_1 = 50000,\ r_2 = 40000,\ h_1 = \tfrac{60000}{7},\ h_2 = \tfrac{80000}{7}'
+++

Two balls united, ten thousand times larger than `scale/balls-united-a-million-from-the-origin`
and at the origin: a ball of radius 50000 centered on the origin, and a ball of radius 40000
centered on (20000, 30000, 60000), 70000 from the first.

If a unit is a millimetre the solid is about 150 metres across: a kernel with a fixed absolute
tolerance meant for parts a few centimetres across must hold it to some twelve significant
figures.

**As a set:**

$$B_{50000} \cup B_{40000}\bigl((20000, 30000, 60000)\bigr)$$
