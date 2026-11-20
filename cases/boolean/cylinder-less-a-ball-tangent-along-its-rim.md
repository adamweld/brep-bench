+++
title = "A cylinder less a ball whose equator is its rim"
tags = ["boolean", "cut", "analytic", "tangent", "coincident", "sphere"]
origin = "OCCT tests/boolean/bopcut_simple/ZH5"

[volume]
value = 167.55160819145564
exact = "π·4²·6 less a hemisphere, (2/3)π·4³"
tex = '\pi r^2 h - \tfrac23\pi r^3 = 96\pi - \tfrac{128\pi}{3}'

[area]
value = 301.59289474462014
exact = "wall 2π·4·6, base π·4², bowl 2π·4²; the top disc is gone, and the wall ends in a knife edge on the rim"
tex = '2\pi r h + \pi r^2 + 2\pi r^2 = 96\pi'
+++

A cylinder of radius 4 and height 6 standing on the xy plane with its axis the z axis, and a ball of radius 4 centred at (0, 0, 6), the middle of the cylinder's top disc. Cut the ball away from the cylinder.

The ball's equator is the cylinder's top rim, where the ball's surface is tangent to the cylinder's wall. The cut takes the whole top disc and a hemisphere's worth below it, leaving a bowl of radius 4 whose wall comes to a knife edge, of no thickness, along the rim.

**As a set:**

$$\{\rho \le 4,\ 0 \le z \le 6\} \setminus B_4\bigl((0, 0, 6)\bigr)$$
