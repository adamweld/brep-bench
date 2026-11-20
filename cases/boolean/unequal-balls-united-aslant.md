+++
title = "Two overlapping balls of very different size, one placed aslant, united"
tags = ["boolean", "union", "analytic", "sphere", "off-axis"]
origin = "OCCT bugs/modalg_7/bug31404"

[volume]
value = 934.0628157548225
exact = "4π(r₁³ + r₂³)/3 − lens, lens = πh₁²(3r₁ − h₁)/3 + πh₂²(3r₂ − h₂)/3, r₁ = 6, r₂ = 2, d = 7, x = (d² + r₁² − r₂²)/2d = 81/14, h₁ = r₁ − x = 3/14, h₂ = r₂ − (d − x) = 11/14"
tex = '\tfrac{4\pi}{3}(r_1^3 + r_2^3) - \tfrac{\pi}{3}h_1^2(3r_1 - h_1) - \tfrac{\pi}{3}h_2^2(3r_2 - h_2),\quad r_1 = 6,\ r_2 = 2,\ d = 7,\ h_1 = \tfrac{3}{14},\ h_2 = \tfrac{11}{14}'

[area]
value = 484.7028665538538
exact = "4π(r₁² + r₂²) − 2πr₁h₁ − 2πr₂h₂"
tex = '4\pi(r_1^2 + r_2^2) - 2\pi r_1 h_1 - 2\pi r_2 h_2'
+++

Two balls: one of radius 6 centred on the origin and one of radius 2 centred at (3, 6, 2), which is 7 from the origin, a little less than the 8 the radii add up to. They overlap in a small lens, but neither holds the other. Make the union.

The line between their centres is off every axis, so the circle where their spheres meet lies in a plane aslant to x, y and z, and cuts only a shallow cap from the big ball.

**As a set:**

$$B_6(0, 0, 0) \cup B_2(3, 6, 2)$$
