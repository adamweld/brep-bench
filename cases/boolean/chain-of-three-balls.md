+++
title = "A chain of three balls"
tags = ["boolean", "union", "sphere", "analytic"]
origin = "brep-conformance 1.0.0, booleans/sequences/sphere-clusters"

[volume]
value = 859.7791094624408
exact = "4π(5³ + 4³ + 3³)/3 − Σ πh²(3r − h)/3 over the four caps: h = 6/7 of the ball of radius 5, 8/7 and 4/5 of the ball of radius 4, 6/5 of the ball of radius 3"
tex = '\tfrac{4}{3}\pi\left(5^3 + 4^3 + 3^3\right) - \sum_{\text{caps}} \tfrac{\pi}{3}h^2(3r - h),\quad (r, h) \in \left\{\left(5, \tfrac{6}{7}\right), \left(4, \tfrac{8}{7}\right), \left(4, \tfrac{4}{5}\right), \left(3, \tfrac{6}{5}\right)\right\}'

[area]
value = 529.9418007655469
exact = "4π(5² + 4² + 3²) − Σ 2πrh over the same four caps"
tex = '4\pi\left(5^2 + 4^2 + 3^2\right) - \sum_{\text{caps}} 2\pi r h'
+++

Three balls united one after another: a ball of radius 5 centered on the origin, a ball of radius
4 centered on (7, 0, 0), and a ball of radius 3 centered on (10, 4, 0). Unite the first two, then
unite the third with that.

The middle ball overlaps both of the others, 7 from the first and 5 from the third, and the first
and the third are 2√29 ≈ 10.77 apart, more than 5 + 3, so they do not touch. The three centers
lie in the plane z = 0, so the solid is symmetric about it.

**As a set:**

$$B_5 \cup B_4\bigl((7, 0, 0)\bigr) \cup B_3\bigl((10, 4, 0)\bigr)$$
