+++
title = "A post on a dome, its side drafted from the dome's base"
tags = ["draft", "cone", "sphere", "boolean", "analytic"]
origin = "OCCT tests/draft/angle/F4"

[volume]
value = 17193.340515548825
exact = "2πR³/3 + π(r₀³ − r₁³)/(3T) − π(R − z₀)²(2R + z₀)/3, R = 20, a = 5, T = tan 3°, z₀ = (aT + √(a²T² + (1 + T²)(R² − a²)))/(1 + T²), r₀ = a − z₀T, r₁ = a − 30T: the half ball, and the frustum of the cone from z₀ to 30 less the cap of the ball above z₀, which it covers"
tex = '\tfrac{2}{3}\pi R^3 + \tfrac{\pi\left(r_0^3 - r_1^3\right)}{3T} - \tfrac{\pi}{3}(R - z_0)^2(2R + z_0),\quad z_0 = \tfrac{aT + \sqrt{a^2T^2 + (1+T^2)(R^2 - a^2)}}{1 + T^2},\ r_0 = a - z_0T,\ r_1 = a - 30T,\ R = 20,\ a = 5,\ T = \tan 3^\circ'

[area]
value = 3998.833092639801
exact = "2πRz₀ + πR² + π(r₀ + r₁)(30 − z₀)/cos 3° + πr₁²: the sphere up to z₀, the base, the cone's side and the top"
tex = '2\pi R z_0 + \pi R^2 + \frac{\pi(r_0 + r_1)(30 - z_0)}{\cos 3^\circ} + \pi r_1^2'
+++

A dome, the half of the ball of radius 20 centered on the origin with z ≥ 0, and a post, the
cylinder of radius 5 about the z axis from z = 0 to z = 30, united. The post's side shows only
above the sphere, from z = √375 ≈ 19.365 to its top at z = 30.

Draft the post's side by 3°, with the pull direction +z and the neutral plane z = 0, the dome's
flat base: the side becomes the cone through the post's circle of radius 5 at z = 0, leaning 3°
from the z direction, inward as it rises, its radius 5 − z·tan 3°. It still runs down into the
dome, now meeting the sphere where (5 − z·tan 3°)² + z² = 400, at z₀ ≈ 19.601, and the top shrinks
to meet it, to radius 5 − 30·tan 3° ≈ 3.428. The dome and the post's top are not drafted.

So the result is the dome with a frustum of the cone standing on it.

**As a set:**

$$\bigl(B_{20} \cap \{\, z \ge 0 \,\}\bigr) \cup \{\, \rho \le 5 - z\tan 3^\circ,\ 0 \le z \le 30 \,\}$$
