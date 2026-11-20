+++
title = "A frustum less the post it stands on"
tags = ["boolean", "cut", "analytic", "cone", "cylinder", "touching"]
origin = "OCCT tests/boolean/boptuc_simple/ZJ4"

[volume]
value = 54.454272662223083
exact = "the frustum itself, untouched: (4π/3)(9 + 3 + 1)"
tex = '\tfrac{4\pi}{3}(9 + 3 + 1)'

[area]
value = 87.614444384223744
exact = "its two ends π·9 + π·1 and its wall π(3 + 1)·√(4² + 2²) = 8√5π"
tex = '9\pi + \pi + 8\sqrt5\,\pi'
+++

A post of radius 5 on the z axis, from z = 0 to z = 6.

A frustum of a cone on the same axis standing on the post's top: from z = 6 to z = 10, radius 3 at z = 6 narrowing in a straight line to radius 1 at z = 10. Cut the post away from the frustum.

The two touch only over the frustum's base disc, which lies in the post's top face; they share no volume, so the result is the frustum as it was.

**As a set:**

$$\{\,6 \le z \le 10,\ \rho \le 3 - \tfrac{z - 6}{2}\,\} \setminus \{\,\rho \le 5,\ 0 \le z \le 6\,\}$$
