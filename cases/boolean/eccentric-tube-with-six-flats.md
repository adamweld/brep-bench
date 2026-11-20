+++
title = "An eccentric tube with six flats and a slanting top"
tags = ["boolean", "cut", "cylinder", "planar", "analytic"]
origin = "OCCT boolean/volumemaker/H9"

[volume]
value = 566.9332079570692
exact = "10·A − 4π(10 + 1.5/4), A as for the cylinder with six flats (π·5² − 6(5²α − 4.5c)): the hole's disk of area 4π has its centre at x = 1.5, where the top is 10 + 1.5/4 high, and the rest is centred on the axis"
tex = '10\,A - 4\pi\left(10 + \tfrac{1.5}{4}\right)'

[area]
value = 551.5416491261936
exact = "(A − 4π)(1 + √17/4) + 12c·10 + 5·10(2π − 12α) + 2π·2·(10 + 1.5/4): the cylinder with six flats' bottom and top with the hole's disk taken out, its flats and its six arcs, and the bore's wall, whose mean height is the top's height over the bore's axis"
tex = '\left(A - 4\pi\right)\left(1 + \tfrac{\sqrt{17}}{4}\right) + 12c\cdot 10 + 5\cdot 10\left(2\pi - 12\alpha\right) + 4\pi\left(10 + \tfrac{1.5}{4}\right)'
+++

A cylinder of radius 5 on the z axis, standing on the plane z = 0, with six flats cut into its side, its top cut slanting, and a hole bored through it off its axis.

The flats are the six planes at distance 4.5 from the z axis, square to the directions at 0°, 60°, 120°, 180°, 240° and 300° about the z axis (the first is the plane x = 4.5), each keeping the side toward the axis; each flat leaves a strip of the cylinder's side between it and the next. The top is the plane z = 10 + x/4. The hole has radius 2 and runs through, along the vertical line x = 1.5, y = 0, so that it lies wholly inside the flats (it reaches 3.5 from the axis at most) and its wall is a cylinder parallel to the outside, 1.5 off its axis.

If you cut the hole with a cylinder, make it run past both faces.

**As a set:**

$$\{\,\rho \le 5,\ x\cos\varphi_k + y\sin\varphi_k \le 4.5\ (\varphi_k = 60^\circ k,\ k = 0, \dots, 5),\ (x - 1.5)^2 + y^2 \ge 2^2,\ 0 \le z \le 10 + x/4\,\}$$
