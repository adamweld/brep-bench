+++
title = "A post less a mid-height block tangent on two sides"
tags = ["boolean", "cut", "cylinder", "tangent", "planar"]
origin = "OCCT tests/boolean/boptuc_simple/ZC7"

[volume]
value = 785.3981633974483
exact = "the post's π·5²·12 = 300π less the half-disc band π·5²/2·4 = 50π"
tex = '\pi\cdot 5^2\cdot 12 - \tfrac{1}{2}\pi\cdot 5^2\cdot 4 = 250\pi'

[area]
value = 589.7787143782138
exact = "the post's 2π·5·12 + 2π·5² = 170π, less the removed wall π·5·4 = 20π, plus the two half-disc ledges 2·(π·5²/2) = 25π and the flat wall 10·4 = 40"
tex = '170\pi - 20\pi + 25\pi + 40 = 175\pi + 40'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to
z = 12. A block with x from 0 to 7, y from −5 to 5 and z from 4 to 8, in the middle of the post's
height, touching neither of its ends. Its faces y = 5 and y = −5 are tangent to the post's wall, each
along the line x = 0, z from 4 to 8; its face x = 0 runs through the post's axis; its face x = 7 is
outside the post. The block covers the half of the post's section where x is positive, for the
middle third of its height. Cut the block from the post: what is left is the post with a half-round
notch through its middle, narrowed there to the half where x ≤ 0.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 12\,\} \setminus \bigl([0,7]\times[-5,5]\times[4,8]\bigr)$$
