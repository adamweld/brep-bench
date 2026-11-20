+++
title = "A square bar cut aslant at both ends, one edge chamfered"
tags = ["boolean", "cut", "analytic", "planar"]
origin = "OCCT boolean/volumemaker/A4, OCCT boolean/volumemaker/A6"

[volume]
value = 592.4
exact = "∬(20 − y/2 − 2z/5) dy dz over the section, the 6 × 6 square less the corner triangle y + z > 10 (leg 2): 592.4"
tex = '\iint \left(20 - \tfrac{y}{2} - \tfrac{2z}{5}\right)dy\,dz = 592.4'

[area]
value = 473.472896739577
exact = "sum of the seven faces' areas, each a planar polygon whose corners are the intersections of three of the planes (done in 40-digit arithmetic)"
tex = '\sum_{i=1}^{7} A_i'
+++

A bar lying along x, bounded by seven planes: y = 0 and y = 6, z = 0 and z = 6 (a square section 6 × 6), the plane y + z = 10, which takes off the long edge where y = 6 meets z = 6 and leaves a face 2 high and 2 wide on its diagonal, and the two ends, which are inclined, not square: the left end is the plane x = 0.4z (it passes through the y axis and leans so that the top, z = 6, is at x = 2.4) and the right end is the plane x = 20 − 0.5y (it is at x = 20 along y = 0 and x = 17 along y = 6).

**As a set:**

$$\{\,0 \le y \le 6,\ 0 \le z \le 6,\ y + z \le 10,\ 0.4z \le x \le 20 - 0.5y\,\}$$
