+++
title = "A large ball less a smaller ball centred on its surface"
tags = ["boolean", "cut", "analytic", "sphere", "off-axis"]
origin = "OCCT bugs/modalg_8/bug33570"

[volume]
value = 877.1670300519189
exact = "4πR³/3 − lens, lens = πh₁²(3R − h₁)/3 + πh₂²(3r − h₂)/3, R = 6, r = 5/2, d = 6, x = (d² + R² − r²)/2d = 263/48, h₁ = R − x = 25/48, h₂ = r − (d − x) = 95/48"
tex = '\tfrac{4\pi}{3}R^3 - \tfrac{\pi}{3}h_1^2(3R - h_1) - \tfrac{\pi}{3}h_2^2(3r - h_2),\quad R = 6,\ r = \tfrac52,\ d = 6,\ h_1 = \tfrac{25}{48},\ h_2 = \tfrac{95}{48}'

[area]
value = 463.843065333143
exact = "4πR² − 2πRh₁ + 2πrh₂: the big sphere less the cap inside the small ball, plus the small sphere's cap inside the big ball"
tex = '4\pi R^2 - 2\pi R h_1 + 2\pi r h_2'
+++

A ball of radius 6 about the origin, and a smaller ball of radius 5/2 centred at (2, 4, 4), which is 6 from the origin, on the big ball's surface. Cut the small ball from the big one.

The small ball bites a dent into the big ball's surface, bounded by a cap of the small ball's surface; the circle where the two spheres meet lies in a plane aslant to x, y and z. 

**As a set:**

$$B_6(0, 0, 0) \setminus B_{5/2}(2, 4, 4)$$
