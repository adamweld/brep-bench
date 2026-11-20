+++
title = "A post united with a block laid at mid-height from its axis, its two long sides tangent to the post"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/bopfuse_simple/ZP3"

[volume]
value = 594.1238596594935
exact = "160π + 4·(48 − 8π): the post and the block's part outside the post (its 6 × 8 plan less the half disc 8π, 4 high)"
tex = '128\pi + 192'

[area]
value = 427.32741228718345
exact = "32π + (80π − 16π) + 2·(48 − 8π) + 2·24 + 32: the post's ends, its wall less the half covered by the block (4π·4), the block's top and bottom outside the post, its sides y = ±4 and its end x = 6"
tex = '80\pi + 176'
+++

A post: a cylinder of radius 4, its axis the z axis, from z = 0 to z = 10.

A block with x from 0 to 6, y from −4 to 4 and z from 3 to 7: it is at mid-height, 3 clear of both ends of the post. Make the union of the post and the block.

The block's end face x = 0 lies in the plane through the post's axis, inside the post. Its side faces y = ±4 each touch the post's wall along a vertical line, x = 0, where block and post are tangent, from z = 3 to z = 7. The block covers the half of the post with x ≥ 0 over that height, and runs out from it to the face x = 6, clear of the post.

**As a set:**

$$\{\,\rho \le 4,\ 0 \le z \le 10\,\} \cup \bigl([0,6]\times[-4,4]\times[3,7]\bigr)$$
