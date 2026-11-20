+++
title = "A cylinder with a square channel cut across it, off its axis"
tags = ["feature", "channel", "cut", "cylinder", "planar", "analytic"]
origin = "OCCT feat/featprism/R7"

[volume]
value = 3752.8509219837921
exact = "πR²H − 6·I, I = ∫₁⁷ 2√(R² − y²) dy = 7√51 − √99 + 100(arcsin 0.7 − arcsin 0.1); R = 10, H = 14"
tex = '\pi R^2 H - 6I,\quad I = \int_1^7 2\sqrt{R^2 - y^2}\,dy = 7\sqrt{51} - \sqrt{99} + 100\left(\arcsin 0.7 - \arcsin 0.1\right)'

[area]
value = 1847.1587626118123
exact = "2πR² + 2πRH − 12R(arcsin 0.7 − arcsin 0.1) [the wall removed at the channel's two mouths] + 6(2√99 + 2√51) [the walls y = 1 and y = 7] + 2I [the floor and the ceiling]"
tex = '2\pi R^2 + 2\pi R H - 12R\left(\arcsin 0.7 - \arcsin 0.1\right) + 12\left(\sqrt{99} + \sqrt{51}\right) + 2I'
+++

A cylinder of radius 10 on the z axis, from z = 0 to z = 14, with a channel of square section cut
right across it. The channel's section is the square 1 ≤ y ≤ 7, 3 ≤ z ≤ 9, and it runs along x from
the cylinder's near wall to its far wall, off the axis, so it leaves by a mouth in the wall at
each end. It is as if a tool began beside the cylinder and was extruded until it reached the cylinder's
curved face on the far side, which is where the channel stops.

**As a set:**

$$\{\,\rho \le 10,\ 0 \le z \le 14\,\} \setminus \bigl(\mathbb{R} \times [1,7] \times [3,9]\bigr)$$
