+++
title = "A post notched at its base by a block, touching it along one line"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/bopcut_simple/Z2"

[volume]
value = 785.39816339744831
exact = "V = 250π − 5A, where A = 0 is the area of the disc of radius 5 inside the block's outline"
tex = '250\pi - 5A,\quad A = 0'

[area]
value = 471.23889803846899
exact = "area = 150π − 5a + 5ℓ (the post's 2π·5² + 2π·5·10, less the wall the block covers, plus the notch's walls), where a = 0 is the length of the post's circle inside the block's outline and ℓ = 0 is the length of the outline's sides inside the disc"
tex = '150\pi - 5a + 5\ell,\quad a = 0,\ \ell = 0'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from 5 to 9, y from −6 to 6 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base. Cut the block away from the post.

The block's face x = 5 touches the post's wall along a line, where block and post are tangent, and the block otherwise stands outside the post, which has no point inside the block. The post's axis is outside the block's outline. The two share no volume.

Nothing is taken from the post.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \setminus \bigl([5,9]\times[-6,6]\times[0,5]\bigr)$$
