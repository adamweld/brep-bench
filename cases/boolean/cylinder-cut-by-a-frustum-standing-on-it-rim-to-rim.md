+++
title = "A cylinder cut by a cone frustum standing on its top, rim to rim"
tags = ["boolean", "cut", "analytic", "coincident", "cone", "touching"]
origin = "OCCT tests/boolean/bopcut_simple/ZK1"

[volume]
value = 785.3981633974483
exact = "the frustum shares only the top disc with the cylinder, so nothing is removed: π·5²·10"
tex = '\pi\cdot 5^2\cdot 10'

[area]
value = 471.23889803846896
exact = "2π·5·10 + 2π·5²"
tex = '2\pi\cdot 5\cdot 10 + 2\pi\cdot 5^2'
+++

A cylinder of radius 5 and height 10 standing on the xy plane, its axis the z axis, and a cone frustum on the same axis standing on the cylinder's top face: its base is the disc of radius 5 in the plane z = 10, its top the disc of radius 2 in the plane z = 14. The frustum's base is the cylinder's top disc exactly, so the two rims coincide and the two bodies touch over a whole face and nothing else. Cut the frustum away from the cylinder: the two share no volume, and the result is the cylinder as it was.

**As a set:**

$$\{\rho \le 5,\ 0 \le z \le 10\} \setminus \{\rho \le 5 - \tfrac34 (z - 10),\ 10 \le z \le 14\}$$
