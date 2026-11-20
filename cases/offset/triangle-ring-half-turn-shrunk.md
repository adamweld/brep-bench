+++
title = "A triangular ring segment, a half turn of it, shrunk by 0.5"
tags = ["offset", "shrink", "revolve", "cone", "sharp-edge", "analytic"]
origin = "OCCT tests/offset/shape_type_i/G6"

[volume]
value = 30.40840800786302
exact = "∫ from z = ½ to z₁ of [D(R₂(z)) − D(R₁(z))] dz, with σ = −√13/6, R₁ = 4 + 2z/3 − σ, R₂ = 8 − 2z/3 + σ, z₁ = 3 + 3σ/2, D(R) = R²(π/2 − arcsin(½/R)) − ½√(R² − ¼): D is the area of the disk of radius R on the side y ≥ ½ of the plane y = ½; each height's slice of the solid is a ring of that disk's side"
tex = '\int_{1/2}^{z_1}\Bigl[D\bigl(R_2(z)\bigr) - D\bigl(R_1(z)\bigr)\Bigr]dz'

[area]
value = 110.0468054751124
exact = "D(R₂(½)) − D(R₁(½)) for the base, plus ∫ (L(R₁) + L(R₂))·√13/3 dz for the two cones, L(R) = R(π − 2 arcsin(½/R)) the arc of a circle of radius R above y = ½, plus 2∫ (√(R₂² − ¼) − √(R₁² − ¼)) dz for the two end faces in the plane y = ½, all over z from ½ to z₁"
tex = 'D\bigl(R_2(\tfrac12)\bigr) - D\bigl(R_1(\tfrac12)\bigr) + \int \tfrac{\sqrt{13}}{3}\bigl(L(R_1) + L(R_2)\bigr)dz + 2\int\Bigl(\sqrt{R_2^2 - \tfrac14} - \sqrt{R_1^2 - \tfrac14}\Bigr)dz'
+++

A ring segment: the triangle in the xz plane with corners (4, 0, 0), (8, 0, 0) and (6, 0, 3), turned
half a turn about the z axis, from the half plane x ≥ 0 through y ≥ 0 to the half plane x ≤ 0. Its
faces are the flat base z = 0 (half of a ring, 4 ≤ ρ ≤ 8), two cones (the triangle's two slanting
sides turned, rising from ρ = 4 and from ρ = 8 to meet at the ring ρ = 6, z = 3), and the two end
triangles, one at x from 4 to 8 and one at x from −8 to −4, both in the plane y = 0 and both facing
−y.

Shrink it by 0.5 with its edges kept sharp: move every face 0.5 inward, against its normal, and
extend the moved faces until they meet. The base moves to z = 0.5; the cones move to the cones
parallel to them, 0.5 nearer in, whose radii at height z are R₁(z) = 4 + 2z/3 − σ and
R₂(z) = 8 − 2z/3 + σ with σ = −√13/6 ≈ −0.6009, meeting at the height z₁ = 3 + 3σ/2 ≈ 2.0986;
both end triangles move to the one plane y = 0.5.

So the shrunk solid is the ring between those cones, from z = 0.5 up to z₁, on the side y ≥ 0.5 of
the plane y = 0.5: it is the half ring y ≥ 0 less its strip 0 ≤ y ≤ 0.5.

**As a set:**

$$\{\, y \ge \tfrac12,\ \tfrac12 \le z \le z_1,\ R_1(z) \le \sqrt{x^2 + y^2} \le R_2(z) \,\}$$
