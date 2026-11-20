+++
title = "A post notched at its base by a block, inscribed in its circle"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/Z4"

[volume]
value = 545.39816339744831
exact = "V = 250π − 5A, where A = 48 is the area of the disc of radius 5 inside the block's outline"
tex = '250\pi - 5A,\quad A = 48'

[area]
value = 611.23889803846899
exact = "area = 150π − 5a + 5ℓ (the post's 2π·5² + 2π·5·10, less the wall the block covers, plus the notch's walls), where a = 0 is the length of the post's circle inside the block's outline and ℓ = 28 is the length of the outline's sides inside the disc"
tex = '150\pi - 5a + 5\ell,\quad a = 0,\ \ell = 28'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −3 to 3, y from −4 to 4 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base. Cut the block away from the post. The block's top face at z = 5 is inside the post, so the cut makes a notch at the post's base and the post is whole above z = 5.

All four corners, (±3, ±4), lie exactly on the post's wall (3² + 4² = 5²): the block's outline is a rectangle inscribed in the post's circle, and the post's axis is at its center. No face of the block is tangent to the post.

Below z = 5 what is left is the part of the post with x < −3 or x > 3 or y < −4 or y > 4; above z = 5 the post is whole. The part left can be in several pieces, even pieces that touch only along a line; build it all as one result, and measure it whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \setminus \bigl([-3,3]\times[-4,4]\times[0,5]\bigr)$$
