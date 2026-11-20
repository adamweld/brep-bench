+++
title = "A quarter of a spherical shell, clipped by a plane"
tags = ["boolean", "cut", "analytic", "sphere", "planar"]
origin = "OCCT boolean/volumemaker/A8, OCCT boolean/volumemaker/B1, OCCT boolean/volumemaker/G7, OCCT boolean/volumemaker/G8"

[volume]
value = 82.205007768932923
exact = "π/4·(∫₀⁴(36 − z²)dz − ∫₀³(9 − z²)dz)"
tex = '\tfrac{\pi}{4}\left(\int_0^4 (36 - z^2)\,dz - \int_0^3 (9 - z^2)\,dz\right)'

[area]
value = 118.7715649669267
exact = "(2π·6·4 + 2π·3·3)/4 + π(36 − 9)/4 + π·20/4 + 2·A_x, A_x = ∫₀⁴√(36 − z²)dz − 9π/4"
tex = '\tfrac14\left(2\pi\cdot 6\cdot 4 + 2\pi\cdot 3\cdot 3\right) + \tfrac{\pi}{4}(36 - 9) + \tfrac{\pi}{4}\cdot 20 + 2A_x,\quad A_x = \int_0^4\sqrt{36 - z^2}\,dz - \tfrac{9\pi}{4}'
+++

A spherical shell between the spheres of radius 3 and radius 6 about the origin, cut to the corner where x ≥ 0, y ≥ 0 and z ≥ 0, and clipped by the plane z = 4. It has four planar faces: x = 0, y = 0, z = 0 and z = 4. The plane z = 4 meets only the outer sphere, in a quarter disc of radius √20, since the inner sphere reaches no higher than z = 3.

**As a set:**

$$\{\,3 \le \lVert p \rVert \le 6,\ x \ge 0,\ y \ge 0,\ 0 \le z \le 4\,\}$$
