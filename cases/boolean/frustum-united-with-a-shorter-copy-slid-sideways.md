+++
title = "A frustum united with a shorter piece of the same cone slid sideways"
tags = ["boolean", "union", "cone", "analytic", "cone-cone"]
origin = "OCCT tests/boolean/bopfuse_simple/ZN2"

[volume]
value = 619.6972642304419
exact = "V₁ + V₂ − ∫₀⁵ L(r(z)) dz, r(z) = 6 − z/2, L(r) = 2r²·arccos(d/2r) − (d/2)·√(4r² − d²) with d = 4 (the lens two discs of radius r make at distance d); V₁ = 8π/3·(36 + 12 + 4), V₂ = 5π/3·(36 + 21 + 12.25)"
tex = '\tfrac{416\pi}{3} + \tfrac{5\pi}{3}\cdot 69.25 - \int_0^5 L(r(z))\,dz'

[area]
value = 470.241336109412
exact = "lateral surfaces by the arcs of each circle outside the other's disc, θ(z) = 2π − 2·arccos(d/2r), ds = (√5/2)dz: 2·∫₀⁵ θ·r·(√5/2) dz + ∫₅⁸ 2π·r·(√5/2) dz; plus the base (2·36π − L(6)), the shorter frustum's top less the lens ((3.5)²π − L(3.5)) and the long frustum's top 4π"
tex = '2\int_0^5 \theta r\tfrac{\sqrt5}{2}\,dz + \int_5^8 2\pi r\tfrac{\sqrt5}{2}\,dz + (72\pi - L(6)) + (\tfrac{49}{4}\pi - L(\tfrac72)) + 4\pi'
+++

A frustum of a cone, its axis the z axis, from z = 0 to z = 8: radius 6 at z = 0, narrowing in a straight line to radius 2 at z = 8, so its radius at height z is 6 − z/2.

A second frustum of the same cone profile, shorter: its axis the vertical line through (4, 0), parallel to the first and 4 to the side, from z = 0 to z = 5, with radius 6 − z/2 at height z (6 at z = 0, 3.5 at z = 5). Make the union of the two.

The two bases lie in the plane z = 0, overlapping in a lens; the two cone surfaces, equal in taper, meet along a curve lying in one plane (x = 2), from the base plane up to the top of the shorter frustum. The shorter frustum's top face, at z = 5, is partly inside the long frustum and partly out of it.

**As a set:**

$$\{\,0 \le z \le 8,\ \rho \le 6 - \tfrac z2\,\} \cup \{\,0 \le z \le 5,\ \sqrt{(x - 4)^2 + y^2} \le 6 - \tfrac z2\,\}$$
