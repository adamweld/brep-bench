+++
title = "A widening cone post on a dome, its side drafted the other way"
tags = ["draft", "cone", "sphere", "boolean", "analytic"]
origin = "OCCT tests/draft/angle/K3, OCCT tests/draft/angle/F3"

[volume]
value = 17342.44035132143
exact = "2πR³/3 + π(r₀³ − r₁³)/(3T) − π(R − z₀)²(2R + z₀)/3, R = 20, a = 6, T = tan 4°, z₀ = (aT + √(a²T² + (1 + T²)(R² − a²)))/(1 + T²), r₀ = a − z₀T, r₁ = a − 30T: the half ball, and the frustum of the cone from z₀ to 30 less the cap of the ball above z₀, which it covers"
tex = '\tfrac{2}{3}\pi R^3 + \tfrac{\pi\left(r_0^3 - r_1^3\right)}{3T} - \tfrac{\pi}{3}(R - z_0)^2(2R + z_0),\quad z_0 = \tfrac{aT + \sqrt{a^2T^2 + (1+T^2)(R^2 - a^2)}}{1 + T^2},\ r_0 = a - z_0T,\ r_1 = a - 30T,\ R = 20,\ a = 6,\ T = \tan 4^\circ'

[area]
value = 4032.8683001295993
exact = "2πRz₀ + πR² + π(r₀ + r₁)(30 − z₀)/cos 4° + πr₁²: the sphere up to z₀, the base, the cone's side and the top"
tex = '2\pi R z_0 + \pi R^2 + \frac{\pi(r_0 + r_1)(30 - z_0)}{\cos 4^\circ} + \pi r_1^2'
+++

The dome: everything within 20 of the origin with z ≥ 0, a hemisphere whose flat face is the disk
z = 0 and whose pole is (0, 0, 20).

A post stands on it: the cone about the z axis from z = 0 to z = 30, its radius 6 at z = 0 and
growing as it rises, 6 + z·tan 2° (about 7.05 at its flat top). It is united with the dome. The
cone's side shows only above the sphere.

Draft the post's side by 4°, with the pull direction +z and the neutral plane z = 0, the dome's
flat base: the side turns about the circle of radius 6 where it crosses the neutral plane, until it
leans 4° from the z direction, inward as it rises. It becomes the cone through that circle with
radius 6 − z·tan 4°, no longer widening but narrowing. The solid to build is this result: the post's wall narrows upward, radius 6 − z·tan 4°; the widening cone above is only a starting point. It still runs down into the dome, now
meeting the sphere where (6 − z·tan 4°)² + z² = 400, at z₀ ≈ 19.454, and the top shrinks to meet
it, to radius 6 − 30·tan 4° ≈ 3.902. The dome and the post's top are not drafted.

So the result is the dome with a frustum of the cone standing on it.

**As a set:**

$$\bigl(B_{20} \cap \{\, z \ge 0 \,\}\bigr) \cup \{\, \rho \le 6 - z\tan 4^\circ,\ 0 \le z \le 30 \,\}$$
