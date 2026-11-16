+++
title = "Two cone frustums base to base, the upper made turned two radians about the axis, their rounded ridge deleted: the two cones meet again"
tags = ["edit", "delete-face", "fillet", "torus", "cone", "analytic"]

[volume]
value = 368.6135380212024
exact = "πh(r₁² + r₁R + R²)/3 + πh(R² + Rr₂ + r₂²)/3 = 196π/3 + 52π = 352π/3, r₁ = 3, R = 5, r₂ = 2, h = 4"
tex = '\tfrac{1}{3}\pi h(r_1^2 + r_1R + R^2) + \tfrac{1}{3}\pi h(R^2 + Rr_2 + r_2^2) = \tfrac{352}{3}\pi'

[area]
value = 263.1934830689617
exact = "πr₁² + π(r₁ + R)√((R − r₁)² + h²) + π(R + r₂)√((R − r₂)² + h²) + πr₂² = 9π + 16√5π + 35π + 4π = 48π + 16√5π"
tex = '\pi r_1^2 + \pi(r_1 + R)\sqrt{(R - r_1)^2 + h^2} + \pi(R + r_2)\sqrt{(R - r_2)^2 + h^2} + \pi r_2^2 = 48\pi + 16\sqrt{5}\pi'
+++

Two cone frustums on the z axis made as one solid: the first widening from radius 3 at z = 0 to
radius 5 at z = 4, the second narrowing from radius 5 at z = 4 to radius 2 at z = 8. Their ridge,
the circle of radius 5 at z = 4 where the two cones meet, is rounded to radius 1: the round is a piece
of a torus tangent to both cones.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The lower
cone runs on up and the upper one down until they meet again in the circle of radius 5 at z = 4.

The result is the two frustums as they were before their ridge was rounded.

**As a set:**

$$\{\,2\sqrt{x^2 + y^2} \le z + 6,\ 0 \le z \le 4\,\} \cup \{\,4\sqrt{x^2 + y^2} + 3z \le 32,\ 4 \le z \le 8\,\}$$

Here the second frustum is made turned by 2.0 radians (114.592°) about the z axis before the two are joined. That changes no point of the solid, but where each surface's own parameter lines start differs between the two surfaces, so the rim where they meet is two edges and its round two faces; both faces are deleted.
