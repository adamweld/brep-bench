+++
title = "A post united with a block at mid-height, two of its faces tangent to the post"
tags = ["boolean", "union", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/bopfuse_simple/ZC6"

[volume]
value = 800.6979416387915
exact = "V = 250π + 4·(36 − 50α), α = arcsin(3/5): the post, and the block's part outside the post's disc (the block's 60 less the 24 + 50α inside it) over its height 4"
tex = '250\pi + 4\,(36 - 50\alpha),\quad \alpha = \arcsin\tfrac35'

[area]
value = 491.4086984556778
exact = "area = 50π (post's ends) + (100π − 20α·4) (post's wall, less the two arcs of 10α inside the block over its height 4) + 2·(36 − 50α) (block's top and bottom outside the disc) + 2·24 (faces x = ±5, whole) + 4·2·2 (faces y = ±3, each with two pieces of length 1 outside the disc, 4 high)"
tex = '50\pi + (100\pi - 80\alpha) + 2\,(36 - 50\alpha) + 48 + 16,\quad \alpha = \arcsin\tfrac35'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 5, y from −3 to 3 and z from 3 to 7: it lies at the middle of the post's height and reaches neither end. Make the union of the post and the block.

The block's faces x = ±5 each touch the post's wall along a vertical line (x = ±5, y = 0, from z = 3 to z = 7), where block and post are tangent. Its faces y = ±3 cross the post's wall at x = ±4. The wall inside the block is covered by it, two curved strips on the post's two sides, and the block's corners stand out of the post.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \bigl([-5,5]\times[-3,3]\times[3,7]\bigr)$$
