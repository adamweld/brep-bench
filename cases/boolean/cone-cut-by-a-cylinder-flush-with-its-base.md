+++
title = "A cone cut by a cylinder on its base"
tags = ["boolean", "cut", "analytic", "coincident", "cone"]
origin = "OCCT tests/boolean/bopcut_simple/ZP8"

[volume]
value = 134.04128655316453
exact = "π·4²·8/3: the cone's upper part, radius 4 at z = 4, apex at z = 12"
tex = '\tfrac13\pi\cdot 4^2\cdot 8'

[area]
value = 162.6625181540883
exact = "π·4² + π·4·√(4² + 8²) = 16π + 16√5·π"
tex = '\pi\cdot 4^2 + \pi\cdot 4\sqrt{4^2 + 8^2} = 16\pi + 16\sqrt5\,\pi'
+++

A cone on the z axis, its base the disc of radius 6 in the plane z = 0 and its apex at (0, 0, 12), less a cylinder of the same radius 6 on the same axis, its base the cone's base disc and its top the disc in the plane z = 4. The cone's side meets the cylinder's wall only along the base rim; above that the cone lies inside the cylinder up to z = 4. The cut takes away the cone below z = 4 and leaves the cone above it: a smaller cone, radius 4 at z = 4, apex at z = 12.

**As a set:**

$$\{\rho \le 6 - \tfrac z2,\ 0 \le z \le 12\} \setminus \{\rho \le 6,\ 0 \le z \le 4\}$$
