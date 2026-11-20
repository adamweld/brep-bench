+++
title = "A cylinder united with a narrow frustum lying across it at right angles, through its axis"
tags = ["boolean", "union", "analytic", "cylinder", "cone", "crossing"]
origin = "OCCT tests/boolean/bopfuse_simple/ZK5"

[volume]
value = 877.4279149258771
exact = "250π + 208π/3 − ∫ A(r(x), w(x)) dx over −5 ≤ x ≤ 5: the cylinder, the frustum (πh/3·(R² + Rr + r²), h = 16, R = 3, r = 1) and the common part, whose section at x is the disc of radius r(x) = 2 − x/8 cut to the strip |y| ≤ w(x) = √(25 − x²): A(r, w) = πr² if w ≥ r, else 2(w√(r² − w²) + r²·arcsin(w/r))"
tex = '250\pi + \tfrac{208\pi}{3} - \int_{-5}^{5} A\bigl(r(x), w(x)\bigr)\,dx'

[area]
value = 557.3764513622377
exact = "150π − W + C + 9π + π: the cylinder's area, less the wall inside the frustum, W = ∫ R·2√(max(r(x)² − y², 0)) dθ over 0 ≤ θ ≤ 2π with (x, y) = 5(cos θ, sin θ), plus the frustum's wall outside the cylinder, C = ∫ r·4·arccos(w/r)·√(65/64) dx over the part of −5 ≤ x ≤ 5 where w < r, together with ∫ 2πr·√(65/64) dx over 5 < |x| ≤ 8, and its two end discs 9π and π"
tex = '150\pi - W + C + 9\pi + \pi'
+++

A cylinder of radius 5, its axis the z axis, from z = 0 to z = 10.

A frustum of a cone lying across it, its axis the line through (0, 0, 5) parallel to the x axis, so that it crosses the cylinder's axis at right angles at the cylinder's mid-height: from x = −8 to x = 8, radius 3 at x = −8, narrowing in a straight line to radius 1 at x = 8, so its radius at x is 2 − x/8. Make the union of the two.

The frustum passes right through the cylinder and comes out of its wall on both sides, its two end faces well clear of it. Its radius never exceeds 3, so it stays between z = 2 and z = 8, clear of the cylinder's ends: the cone's surface meets the cylinder's wall in two closed curves, one where it enters and one where it leaves.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \{\,-8 \le x \le 8,\ y^2 + (z - 5)^2 \le (2 - \tfrac x8)^2\,\}$$
