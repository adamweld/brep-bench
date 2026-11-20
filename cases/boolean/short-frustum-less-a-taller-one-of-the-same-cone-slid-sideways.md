+++
title = "A short frustum less a taller piece of the same cone slid sideways"
tags = ["boolean", "cut", "cone", "analytic", "cone-cone"]
origin = "OCCT tests/boolean/boptuc_simple/ZN2"

[volume]
value = 184.06308293265721
exact = "V₂ − ∫₀⁵ L(r(z)) dz, r(z) = 6 − z/2, L(r) = 2r²·arccos(d/2r) − (d/2)·√(4r² − d²) with d = 4 (the lens two discs of radius r make at distance d); V₂ = 5π/3·(36 + 21 + 12.25)"
tex = '\tfrac{5\pi}{3}\cdot 69.25 - \int_0^5 L(r(z))\,dz'

[area]
value = 240.32616631725975
exact = "the short frustum's wall, 9.5π·(5√5/2) (at each height the arc of it outside the taller cone and the arc of the taller cone's wall inside the short one are equal in length, so together they make the short wall's full circle), plus its base less the lens (36π − L(6)) and its top less the lens (12.25π − L(3.5))"
tex = '9.5\pi\cdot\tfrac{5\sqrt5}{2} + (36\pi - L(6)) + (\tfrac{49}{4}\pi - L(\tfrac72))'
+++

A frustum of a cone, its axis the z axis, from z = 0 to z = 8: radius 6 at z = 0, narrowing in a straight line to radius 2 at z = 8, so its radius at height z is 6 − z/2.

A second frustum of the same cone profile, shorter: its axis the vertical line through (4, 0), parallel to the first and 4 to the side, from z = 0 to z = 5, with radius 6 − z/2 at height z (6 at z = 0, 3.5 at z = 5). Cut the first frustum away from the second.

The bases lie in the plane z = 0, overlapping in a lens; the two cone surfaces, equal in taper, meet along a curve lying in one plane (x = 2), from the base plane up to the top of the shorter frustum. What is left of the shorter frustum is its crescent-shaped part on the far side of that curve, in x > 2 at the base.

**As a set:**

$$\{\,0 \le z \le 5,\ \sqrt{(x - 4)^2 + y^2} \le 6 - \tfrac z2\,\} \setminus \{\,0 \le z \le 8,\ \rho \le 6 - \tfrac z2\,\}$$
