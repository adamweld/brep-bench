+++
title = "Three quarters of a cone frustum, both its ends moved out"
tags = ["edit", "move-face", "cone", "concave", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/J7"

[volume]
value = 2376.4184928998293
exact = "(3/4)·πh(r₁² + r₁r₂ + r₂²)/3 = (π/4)·19·159.25, h = 19, r₁ = 11.5, r₂ = 2"
tex = '\tfrac{3}{4}\cdot\tfrac{\pi h}{3}\left(r_1^2 + r_1 r_2 + r_2^2\right) = \tfrac{\pi}{4}\cdot 19\cdot 159.25,\quad h = 19,\ r_1 = 11.5,\ r_2 = 2'

[area]
value = 1253.2308662306868
exact = "(3/4)(π(r₁ + r₂)ℓ + πr₁² + πr₂²) + 2·(r₁ + r₂)h/2, ℓ = √(h² + (r₁ − r₂)²) = 9.5√5: three quarters of the cone and of the two ends, and the two flats, trapezoids"
tex = '\tfrac{3}{4}\left(\pi(r_1 + r_2)\ell + \pi r_1^2 + \pi r_2^2\right) + 2\cdot\tfrac{(r_1 + r_2)h}{2},\quad \ell = \sqrt{h^2 + (r_1 - r_2)^2} = 9.5\sqrt{5}'
+++

Three quarters of a cone frustum about the z axis, of radius 10 at z = 0 and 4 at z = 12 (its cone's
apex would be at z = 20): the part whose angle about the z axis, φ, measured from +x toward +y,
lies between 0° and 270°. Its faces are the cone; a bottom and a top, three quarters of disks of
radius 10 and 4; and two flat trapezoids, in the half-planes φ = 0° and φ = 270°, which meet along
the axis in a concave edge.

Then edit the solid directly: move the top 4 up and the bottom 3 down, each along its outward
normal, to the planes z = 16 and z = −3. The cone and the two flats run on to meet them, so the top
shrinks to radius 2 and the bottom grows to radius 11.5.

So the result is three quarters of the frustum of the same cone from z = −3, radius 11.5, to
z = 16, radius 2.

**As a set:**

$$\{\, \rho \le 10\left(1 - \tfrac{z}{20}\right),\ -3 \le z \le 16,\ 0^\circ \le \varphi \le 270^\circ \,\}$$
