+++
title = "A triangle turned half a turn about an axis beside its plane"
tags = ["primitive", "revolve", "analytic", "hyperboloid", "skew-axis"]
origin = "OCCT modalg_1/bug1226"

[volume]
value = 87.96459430051421
exact = "28π = π·A·ū: the triangle's area 12 times the distance 7/3 of its centroid from the line the axis projects to in its plane. Each height z contributes the region where, for each distance ρ from the axis, the angles run over a half turn, so π/2·(ρ₂² − ρ₁²) = π/2·(u₂² − u₁²) there"
tex = '\pi A\,\bar u = \pi\cdot 12\cdot\tfrac73 = 28\pi'

[area]
value = 242.84991439298673
exact = "24 + 12π + 6π√17 + (3π/2)·∫₁⁵ √(16 + 13u²/9) du: the two triangles (12 each), the floor's half annulus (π/2)(25 − 1) = 12π, the half cylinder of the side parallel to the axis (π·√17·6), and the hyperboloid, π∫ρ·√(1 + ρ'²) dz with ρ² = 16 + u², u = 5 − 2z/3. The integral is elementary: (u/2)·√(16 + 13u²/9) + (24/√13)·asinh(√13·u/12), from u = 1 to 5"
tex = '24 + 12\pi + 6\pi\sqrt{17} + \tfrac{3\pi}{2}\int_1^5 \sqrt{16 + \tfrac{13}{9}u^2}\,du'
+++

In the plane x = 4, take the right triangle with corners (4, 1, 0), (4, 5, 0) and (4, 1, 6). Turn it
half a turn, 180°, about the z axis, right-handed: from +x toward +y. The z axis is parallel to the
triangle's plane, 4 away from it, and it is skew to the triangle's slanted side, from (4, 5, 0) to
(4, 1, 6): the two lines never meet and are not parallel. The solid is everything the triangle
passes through.

Its faces are the triangle where it starts and the same triangle turned into the plane x = −4,
where it ends; the half cylinder (radius √17) that the side parallel to the z axis sweeps; the
flat floor that the side at z = 0 sweeps; and the slanted side's surface, a piece of a hyperboloid
of one sheet.

**As a set:** with $u = \sqrt{\rho^2 - 16}$ and $\varphi$ the angle about the z axis from +x toward +y,

$$\{\,0 \le z \le 6,\ 1 \le u \le 5 - \tfrac{2z}{3},\ 0 \le \varphi - \arccos(4/\rho) \le \pi \pmod{2\pi}\,\}$$
