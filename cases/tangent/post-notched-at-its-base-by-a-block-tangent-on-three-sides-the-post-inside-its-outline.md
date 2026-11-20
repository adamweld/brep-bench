+++
title = "A post notched at its base by a block, tangent on three sides, the post inside its outline"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/bopcut_simple/W1"

[volume]
value = 392.69908169872415
exact = "V = 250π − 5A, where A = 25·π is the area of the disc of radius 5 inside the block's outline"
tex = '250\pi - 5A,\quad A = 25 \pi'

[area]
value = 314.15926535897932
exact = "area = 150π − 5a + 5ℓ (the post's 2π·5² + 2π·5·10, less the wall the block covers, plus the notch's walls), where a = 10·π is the length of the post's circle inside the block's outline and ℓ = 0 is the length of the outline's sides inside the disc"
tex = '150\pi - 5a + 5\ell,\quad a = 10 \pi,\ \ell = 0'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 5, y from −5 to 7 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base. Cut the block away from the post. The block's top face at z = 5 is inside the post, so the cut makes a notch at the post's base and the post is whole above z = 5.

The block's faces x = ±5 and y = −5 each touch the post's wall along a line, where block and post are tangent. Its face y = 7 stands clear of the post, so the post lies wholly inside the block's outline. All four corners are outside the post.

Below z = 5 nothing is left of the post; above z = 5 the post is whole. The part left can be in several pieces, even pieces that touch only along a line; build it all as one result, and measure it whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \setminus \bigl([-5,5]\times[-5,7]\times[0,5]\bigr)$$
