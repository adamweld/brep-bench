+++
title = "A cylinder with six flats and a slanting top"
tags = ["boolean", "cut", "cylinder", "planar", "analytic"]
origin = "OCCT boolean/volumemaker/D8"

[volume]
value = 697.3093030810456
exact = "10·A, A = π·5² − 6(5²α − 4.5c), α = arccos(4.5/5), c = √(5² − 4.5²): the section is the disk less six segments, and its centroid is on the axis, where the top is 10 high"
tex = '10\,A,\quad A = \pi\cdot 5^2 - 6\left(5^2\alpha - 4.5\,c\right),\ \alpha = \arccos\tfrac{4.5}{5},\ c = \sqrt{5^2 - 4.5^2}'

[area]
value = 446.68504295999156
exact = "A(1 + √17/4) + 12c·10 + 5·10(2π − 12α): the bottom, the top (the section over cos θ = 4/√17), the six flats (each 2c wide and, as the top is flat-sloped, as high on average as at its middle, 10 + 4.5cos(60k°)/4, which sum to 60 over the six) and the six arcs of side, 2π − 12α of angle in all"
tex = 'A\left(1 + \tfrac{\sqrt{17}}{4}\right) + 12c\cdot 10 + 5\cdot 10\left(2\pi - 12\alpha\right)'
+++

A cylinder of radius 5 on the z axis, standing on the plane z = 0, with six flats cut into its side and its top cut slanting.

The flats are the six planes at distance 4.5 from the z axis, square to the directions at 0°, 60°, 120°, 180°, 240° and 300° about the z axis (the first is the plane x = 4.5), each keeping the side toward the axis. Each flat takes a strip off the side with a chord of 2√4.75 ≈ 4.36, and between every two flats a strip of the cylinder's side is left. The solid stands from z = 0 up to the plane z = 10 + x/4, so its top slants, 10 high over the axis, 11.125 high over the flat at x = 4.5, and 8.875 high over the one opposite.

**As a set:**

$$\{\,\rho \le 5,\ x\cos\varphi_k + y\sin\varphi_k \le 4.5\ (\varphi_k = 60^\circ k,\ k = 0, \dots, 5),\ 0 \le z \le 10 + x/4\,\}$$
