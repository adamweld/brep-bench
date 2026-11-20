+++
title = "A short cylinder less a ball of its own radius centred beyond its rim and above its top"
tags = ["boolean", "cut", "sphere", "cylinder"]
origin = "OCCT bugs/modalg_8/bug33541"
settled = false
+++

A cylinder of radius 4 and height 3 standing on the xy plane, its axis the z axis, and a ball of the same radius, 4, centred at (6, 0, 5), outside the cylinder: farther from its axis than its rim, and above its top. Cut the ball from the cylinder.

The ball reaches down to z = 1 and in to ρ = 2 along the x axis, so it bites a notch from the cylinder's top and rim near the point (4, 0, 3), its surface crossing the cylinder's top face and its wall.

**As a set:**

$$\{\,\rho \le 4,\ 0 \le z \le 3\,\} \setminus B_4(6, 0, 5)$$
