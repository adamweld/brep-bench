+++
title = "A pin touching a drum's wall from inside, united"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "tangent-line", "seam"]
origin = "OCCT tests/boolean/bfuse_complex/J1"

[volume]
value = 3311.238656883642
exact = "πR²H + πr²·6 = 1000π + 54π = 1054π, R = 10, H = 10, r = 3: the drum and the 6 of the pin above it"
tex = '\pi R^2 H + 6\pi r^2 = 1054\pi,\quad R = 10,\ H = 10,\ r = 3'

[area]
value = 1369.73439696515
exact = "2πRH + πR² + (πR² − πr²) + 2πr·6 + πr² = 200π + 100π + 91π + 36π + 9π = 436π: the drum's wall, bottom and top less the pin's disk, and the pin's wall and top above it"
tex = '2\pi R H + \pi R^2 + \left(\pi R^2 - \pi r^2\right) + 12\pi r + \pi r^2 = 436\pi'
+++

A cylinder (a drum) of radius 10 on the z axis, from z = 0 to z = 10, united with a thinner cylinder
(a pin) of radius 3 on the upright line x = 7, y = 0, from z = 4 to z = 16.

The pin lies inside the drum up to z = 10 and touches the drum's wall from inside along the line
x = 10, y = 0, where the two walls are tangent; above the drum's top it stands up 6 more. The union
is the drum with the pin standing out of its top. Inside the drum the pin's wall and bottom go, and
the drum's wall is whole: the line of contact is not an edge of the result. The drum's top is left
as its disk less the pin's, the two circles touching at (10, 0, 10), so that the top face narrows
to nothing there from either side; from that point the pin's wall rises on, tangent to where the
drum's wall would run.

This case is about two curved faces tangent along a line. If your cylinders have seams, put each on
the line of contact, x = 10, y = 0, on the +x side of its axis.

**As a set:**

$$\{\,\rho \le 10,\ 0 \le z \le 10\,\} \cup \{\,(x - 7)^2 + y^2 \le 3^2,\ 4 \le z \le 16\,\}$$
