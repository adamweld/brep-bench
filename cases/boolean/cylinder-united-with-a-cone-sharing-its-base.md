+++
title = "A cylinder united with a cone that stands on the same base circle and rises through its top"
tags = ["boolean", "union", "analytic", "cylinder", "cone", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZP4"

[volume]
value = 264.5660578726812
exact = "80π + π·r₅²·4/3, r₅ = 16/9: the cylinder and the tip of the cone above its top, a cone of height 4"
tex = '80\pi + \tfrac{\pi}{3}\bigl(\tfrac{16}{9}\bigr)^2\cdot 4'

[area]
value = 240.7129752180811
exact = "16π + 40π + (16 − r₅²)π + π·r₅·√(r₅² + 16): the cylinder's base and wall, its top ring, and the tip's wall"
tex = '16\pi + 40\pi + \bigl(16 - \tfrac{256}{81}\bigr)\pi + \tfrac{64\sqrt{97}}{81}\pi'
+++

A cone: its axis the z axis, its base the disc of radius 4 in the plane z = 0, its apex at (0, 0, 9).

A cylinder of radius 4, its axis the z axis, from z = 0 to z = 5. It has the cone's base circle as its own base, so the cone stands inside it, its base disc the cylinder's, its base rim the cylinder's rim. Make the union of the two.

The cone is clear of the cylinder's wall everywhere except along the shared rim. It leaves the cylinder's top face at z = 5 in a circle of radius 16/9, and the union is the cylinder with the tip of the cone, 4 high, standing on its top.

**As a set:**

$$\{\,\rho \le 4,\ 0 \le z \le 5\,\} \cup \{\,0 \le z \le 9,\ \rho \le 4\,(1 - \tfrac z9)\,\}$$
