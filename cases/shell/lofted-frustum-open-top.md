+++
title = "A frustum lofted between two circles, shelled with its narrow top open"
tags = ["shell", "inward", "loft", "cone", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug5805_30"

[volume]
value = 283.66391426909206
exact = "168π − (7π/3)(a² + ab + b²), a = 45/8 − k, b = 3 − k, k = √73/8: the frustum, πH(R² + Rr + r²)/3 = 168π, less the cavity, the frustum from z = 1 to z = 8 whose cone is the side moved 1 inward, its radius less by 1/cos α = k, tan α = 3/8"
tex = '168\pi - \tfrac{7\pi}{3}\left(a^2 + ab + b^2\right),\quad a = \tfrac{45}{8} - k,\ b = 3 - k,\ k = \tfrac{\sqrt{73}}{8}'

[area]
value = 588.8646336520272
exact = "36π + 9π√73 + π(9 − b²) + πa² + π(a + b)·7√73/8, a and b as above: the bottom, the side, the rim at z = 8, the cavity's floor and its cone"
tex = '36\pi + 9\pi\sqrt{73} + \pi\left(9 - b^2\right) + \pi a^2 + \pi(a + b)\,\tfrac{7\sqrt{73}}{8}'
+++

A cone frustum on the z axis made by a loft: its side lofted, ruled, between the circle of radius
6 at z = 0 and the circle of radius 3 at z = 8, and closed by the two disks they bound. The ruled
surface between two coaxial circles is the cone through them, so the frustum's radius is
ρ = 6 − 3z/8, its side leaning α = atan(3/8) from the z direction.

Shell it inward to a wall 1 thick, open at its narrow top face, z = 8. The outside of the wall is
its side and its bottom, unmoved, and the inside is each of them moved 1 inward along its normal,
onto the surface offset 1 from it. The moved faces are extended or trimmed where they meet one
another, and run on until they meet the surface of the opening, which is left as a rim between
the outside and the inside.

The side, moved 1 inward, is the cone of the same angle whose radius is less by
1/cos α = √73/8 ≈ 1.068: ρ = 6 − 3z/8 − √73/8. The bottom moved is the plane z = 1. So the result is
the frustum less the frustum between z = 1 and z = 8 under that cone, of radius 45/8 − √73/8 ≈ 4.557
at z = 1 and 3 − √73/8 ≈ 1.932 at z = 8: a cup whose rim is the annulus between radii 1.932 and 3
at z = 8.

**As a set:**

$$\{\, \rho \le 6 - \tfrac{3z}{8},\ 0 \le z \le 8 \,\} \setminus \{\, \rho \le 6 - \tfrac{3z}{8} - \tfrac{\sqrt{73}}{8},\ z \ge 1 \,\}$$
