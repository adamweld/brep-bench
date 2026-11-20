+++
title = "A frustum through a post, less the post: its two ends"
tags = ["boolean", "cut", "analytic", "cone", "cylinder", "pieces"]
origin = "OCCT tests/boolean/boptuc_simple/ZJ9"

[volume]
value = 86.393797973719314
exact = "two frustums of the cone r = 5/2 − z/6: z from −3 to 0 (radius 3 to 5/2), π(9 + 15/2 + 25/4) = 91π/4, and z from 6 to 9 (radius 3/2 to 1), π(9/4 + 3/2 + 1) = 19π/4"
tex = '\pi(9 + \tfrac{15}{2} + \tfrac{25}{4}) + \pi(\tfrac94 + \tfrac32 + 1)'

[area]
value = 134.55771240627577
exact = "the lower piece's ends 9π and 25π/4 and wall π(3 + 5/2)·√37/2; the upper piece's ends 9π/4 and π and wall π(3/2 + 1)·√37/2"
tex = '(9 + \tfrac{25}{4} + \tfrac94 + 1)\pi + 4\sqrt{37}\,\pi'
+++

A post of radius 5 on the z axis, from z = 0 to z = 6.

A frustum of a cone on the same axis, longer than the post and passing out of both its ends: from z = −3 to z = 9, radius 3 at z = −3 narrowing in a straight line to radius 1 at z = 9 (radius 5/2 − z/6 at height z). Cut the post away from the frustum.

The frustum is narrower than the post wherever they overlap, so its wall is clear of the post's and only the post's two end planes cut it. What is left is two separate pieces: the part below z = 0 (radius 3 to 5/2) and the part above z = 6 (radius 3/2 to 1).

**As a set:**

$$\{\,-3 \le z \le 9,\ \rho \le \tfrac52 - \tfrac z6\,\} \setminus \{\,\rho \le 5,\ 0 \le z \le 6\,\}$$
