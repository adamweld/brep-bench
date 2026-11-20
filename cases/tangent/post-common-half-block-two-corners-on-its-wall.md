+++
title = "A post and a block half as tall with two corners on its wall, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcommon_simple/Z6"

[volume]
value = 260.437638599160548
exact = "V = 5·A, where A = 36 + 25·arcsin(3/5) is the area of the disc of radius 5 inside the block's outline"
tex = '5A,\quad A = 36 + 25\arcsin\tfrac35'

[area]
value = 246.350110879328439
exact = "area = 2A + 5(a + ℓ), where a = 10·arcsin(3/5) is the length of the post's circle inside the block's outline and ℓ = 22 is the length of the outline's sides inside the disc"
tex = '2A + 5(a + \ell),\quad a = 10\arcsin\tfrac35,\ \ell = 22'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −3 to 3, y from −4 to 9 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base, its top face at z = 5 inside the post. Take the part common to the post and the block.

The block's corners (±3, −4) lie exactly on the post's wall (3² + 4² = 5²) and its corners (±3, 9) are far outside. The faces x = ±3 and y = −4 cut across the post's circle; no face is tangent to it. The post's axis is inside the block's outline.

What is left is the part of the post with −3 ≤ x ≤ 3 and y ≥ −4, from z = 0 to z = 5: three flat sides (x = −3, x = 3, y = −4) and a patch of the post's wall over the top, with flat top and bottom.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \bigl([-3,3]\times[-4,9]\times[0,5]\bigr)$$
