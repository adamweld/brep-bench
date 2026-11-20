+++
title = "A cone frustum less a torus centred in its top face, the torus's outer equator on its rim"
tags = ["boolean", "cut", "analytic", "cone", "torus", "tangent", "coincident"]
origin = "OCCT bugs/modalg_7/bug31836"

[volume]
value = 1081.968230199342
exact = "π·8/3·(6² + 6·8 + 8²) − π²Rr² = 1184π/3 − 16π², R = 4, r = 2: the frustum less the torus's lower half"
tex = '\tfrac{1184\pi}{3} - \pi^2 R r^2 = \tfrac{1184\pi}{3} - 16\pi^2'

[area]
value = 734.2292844771611
exact = "the frustum's base 64π and side π(6 + 8)√(8² + 2²) = 14π√68, the disc ρ ≤ 2 left of its top, π(R − r)² = 4π, and the torus's lower half now bounding a groove, 2π²Rr = 16π²"
tex = '64\pi + 14\pi\sqrt{68} + 4\pi + 16\pi^2'
+++

A cone frustum on the z axis, from z = −8 to z = 0, radius 8 at z = −8 and radius 6 at z = 0, so that its radius at height z is $6 - z/4$. And a torus on the same axis, centred on the origin, which is in the plane of the frustum's top face: its tube has radius 2 and its centre circle has radius 4 in the plane z = 0. The torus's outer equator, the circle of radius 6 in the plane z = 0, is exactly the frustum's top rim; its inner equator is the circle of radius 2 there.

The frustum widens downward, so the torus's lower half lies inside it, touching nothing of its wall; the torus's upper half lies above its top face, which it meets in the ring 2 ≤ ρ ≤ 6.

Cut the torus away from the frustum. Its lower half leaves a ring groove open at the top, running to the rim, where the groove's outer wall comes to nothing against the cone: the top face is left as only the disc ρ ≤ 2, and the cone's side runs up to an edge, along the rim, where it meets the groove's surface.

**As a set:**

$$\{\,\rho \le 6 - \tfrac{z}{4},\ -8 \le z \le 0\,\} \setminus \{\,(\rho - 4)^2 + z^2 \le 2^2\,\}$$
