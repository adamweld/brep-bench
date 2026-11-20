+++
title = "A frustum with a conical pocket flush at its top"
tags = ["boolean", "cut", "cone", "coplanar", "blind"]
origin = "OCCT tests/boolean/bopcut_simple/ZM2"

[volume]
value = 498.46603436958053
exact = "πh/3·(R² + Rr + r²) for the frustum, 8π/3·(36 + 18 + 9) = 168π, less the pocket's 4π/3·(4 + 2 + 1) = 28π/3: 476π/3"
tex = '\tfrac{8\pi}{3}(36 + 18 + 9) - \tfrac{4\pi}{3}(4 + 2 + 1) = \tfrac{476\pi}{3}'

[area]
value = 412.382261067816
exact = "outer side π(6 + 3)√(8² + 3²) = 9π√73, base 36π, top ring π(3² − 2²) = 5π, pocket side π(1 + 2)√(4² + 1²) = 3π√17, floor π"
tex = '9\pi\sqrt{73} + 36\pi + 5\pi + 3\pi\sqrt{17} + \pi'
+++

A frustum standing on the xy plane, its axis the z axis: a disc of radius 6 at z = 0 for its base,
a disc of radius 3 at z = 8 for its top, the cone between them.

A pocket sunk into its top: a second, narrower frustum on the same axis, from a disc of radius 1 at
z = 4 up to a disc of radius 2 at z = 8, so that it widens toward the top. Its top disc lies in the
same plane as the frustum's top disc, z = 8, and inside it. Cut the pocket from the frustum. What is
left is a frustum with a blind conical hole, open at the top, whose floor is the disc of radius 1 at
z = 4; the top face is a ring between radius 2 and radius 3.

**As a set:**

$$\{\,\rho \le 6 - \tfrac38 z,\ 0 \le z \le 8\,\} \setminus \{\,\rho \le \tfrac z4,\ 4 \le z \le 8\,\}$$
