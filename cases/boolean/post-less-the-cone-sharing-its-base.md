+++
title = "A post less the cone that shares its base"
tags = ["boolean", "cut", "analytic", "cone", "cylinder", "coincident"]
origin = "OCCT tests/boolean/boptuc_simple/ZP3"

[volume]
value = 83.77580409572782
exact = "post 64π less the cone's frustum up to z = 4: (4π/3)(16 + 8 + 4) = 112π/3"
tex = '16\pi\cdot 4 - \tfrac{4\pi}{3}(16 + 8 + 4)'

[area]
value = 222.52785353043962
exact = "the post's wall 2π·4·4 = 32π, its top ring π(16 − 4) = 12π, and the cone's wall up to z = 4: π(4 + 2)·√(4² + 2²) = 12√5π; the base is gone, the cone's base filling it"
tex = '32\pi + 12\pi + 12\sqrt5\,\pi'
+++

A cone on the z axis: its base the disc of radius 4 in the plane z = 0, its apex at (0, 0, 8).

A post on the same axis: radius 4, from z = 0 to z = 4. Its bottom face is the cone's base disc, and its rim is the cone's rim. Cut the cone away from the post.

The cone's wall runs inside the post from the shared rim at z = 0 up to radius 2 at z = 4. What is left is a ring: a wedge that is thin as a knife edge at the bottom rim and 2 thick at the top, with no bottom face; its top is a flat ring of radii 2 and 4.

**As a set:**

$$\{\,\rho \le 4,\ 0 \le z \le 4\,\} \setminus \{\,0 \le z \le 8,\ \rho \le 4 - \tfrac z2\,\}$$
